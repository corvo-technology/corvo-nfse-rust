//! Invoices resource.

use serde_json::json;

use crate::error::CorvoError;
use crate::resources::companies::encode;
use crate::transport::{push_query, DownloadedFile, Paginator, RequestSpec, Unwrap};
use crate::types::common::Paginated;
use crate::types::invoice::{
    DownloadFormat, Invoice, InvoiceBatchItem, InvoiceBatchSummary, InvoiceCalculation,
    InvoiceCancelParams, InvoiceCreateParams, InvoiceDownloadUrl, InvoiceEmailResult, InvoiceEvent,
    InvoiceListParams,
};

/// Invoices resource, obtained via [`Corvo::invoices()`](crate::Corvo::invoices).
pub struct Invoices<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> Invoices<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Issue an invoice (`POST /invoices`).
    ///
    /// `idempotency_key` is sent as the `Idempotency-Key` header when provided;
    /// retries with the same key never duplicate an invoice.
    pub async fn create(
        &self,
        params: &InvoiceCreateParams,
        idempotency_key: Option<&str>,
    ) -> Result<Invoice, CorvoError> {
        let spec = RequestSpec::post("/invoices")
            .json(params)?
            .idempotency_key(idempotency_key.map(str::to_owned));
        self.transport.request_json(&spec).await
    }

    /// Issue 1 to 50 invoices in one call (`POST /invoices/batches`), for one
    /// company or several.
    ///
    /// Items are checked one by one: valid ones are saved together, invalid ones
    /// come back `rejected` and are not saved. Match `results[].index` to the
    /// item you sent, fix only the rejected ones and send them again. Send an
    /// `idempotency_key` per item; the `Idempotency-Key` header is not used by
    /// this route.
    pub async fn create_batch(
        &self,
        invoices: &[InvoiceBatchItem],
    ) -> Result<InvoiceBatchSummary, CorvoError> {
        let spec = RequestSpec::post("/invoices/batches").json(&json!({ "invoices": invoices }))?;
        self.transport.request_json(&spec).await
    }

    /// Estimate the taxes of an invoice without issuing, storing or charging
    /// anything (`POST /invoices/calculate`).
    ///
    /// A fiscal gap is not an error: the result has `status == "blocked"`.
    pub async fn calculate(
        &self,
        params: &InvoiceCreateParams,
    ) -> Result<InvoiceCalculation, CorvoError> {
        let spec = RequestSpec::post("/invoices/calculate").json(params)?;
        self.transport.request_json(&spec).await
    }

    /// Fetch one invoice (`GET /invoices/:id`).
    pub async fn get(&self, id: &str) -> Result<Invoice, CorvoError> {
        let spec = RequestSpec::get(&format!("/invoices/{}", encode(id)));
        self.transport.request_json(&spec).await
    }

    /// List invoices (`GET /invoices`).
    pub async fn list(&self, params: &InvoiceListParams) -> Result<Paginated<Invoice>, CorvoError> {
        self.transport
            .list_page("/invoices", &list_query(params), Unwrap::None)
            .await
    }

    /// Stream every invoice across pages (`GET /invoices`).
    ///
    /// The first page failure surfaces as the first `Err` from the stream.
    pub fn iterate(&self, params: &InvoiceListParams) -> Paginator<'c, Invoice> {
        Paginator::new(
            self.transport,
            "/invoices",
            list_query(params),
            Unwrap::None,
            params.cursor.clone(),
        )
    }

    /// List status events of an invoice (`GET /invoices/:id/events`).
    pub async fn events(
        &self,
        id: &str,
        limit: Option<u16>,
        cursor: Option<&str>,
    ) -> Result<Paginated<InvoiceEvent>, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "limit", limit.map(|l| l.to_string()));
        push_query(&mut query, "cursor", cursor.map(str::to_owned));
        self.transport
            .list_page(
                &format!("/invoices/{}/events", encode(id)),
                &query,
                Unwrap::None,
            )
            .await
    }

    /// Download the invoice PDF (`GET /invoices/:id/pdf`).
    pub async fn pdf(&self, id: &str) -> Result<DownloadedFile, CorvoError> {
        let spec = RequestSpec::get(&format!("/invoices/{}/pdf", encode(id)));
        self.transport.request_raw(&spec).await
    }

    /// Download the invoice XML (`GET /invoices/:id/xml`).
    pub async fn xml(&self, id: &str) -> Result<DownloadedFile, CorvoError> {
        let spec = RequestSpec::get(&format!("/invoices/{}/xml", encode(id)));
        self.transport.request_raw(&spec).await
    }

    /// Get a pre-signed download URL (`POST /invoices/:id/download-url`).
    pub async fn download_url(
        &self,
        id: &str,
        format: DownloadFormat,
    ) -> Result<InvoiceDownloadUrl, CorvoError> {
        let spec = RequestSpec::post(&format!("/invoices/{}/download-url", encode(id)))
            .json(&json!({ "format": format.as_str() }))?;
        self.transport.request_json(&spec).await
    }

    /// Re-send the invoice by email (`POST /invoices/:id/email`).
    pub async fn send_email(
        &self,
        id: &str,
        idempotency_key: Option<&str>,
    ) -> Result<InvoiceEmailResult, CorvoError> {
        let spec = RequestSpec::post(&format!("/invoices/{}/email", encode(id)))
            .idempotency_key(idempotency_key.map(str::to_owned));
        self.transport.request_json(&spec).await
    }

    /// Cancel an invoice (`POST /invoices/:id/cancel`) and return it in its
    /// new state.
    ///
    /// * A `queued` invoice is cancelled right away: `status` is
    ///   [`InvoiceStatus::Cancelled`](crate::InvoiceStatus::Cancelled).
    /// * An `authorized` invoice moves to
    ///   [`InvoiceStatus::Cancelling`](crate::InvoiceStatus::Cancelling) while
    ///   the Sefin processes the cancellation event.
    /// * A `processing` invoice stays `processing` with
    ///   `cancellation_requested_at` set; it is cancelled once authorized.
    ///
    /// Follow the outcome with [`get`](Self::get) (or the
    /// `nfse.invoice.cancelled` / `nfse.invoice.cancellation_rejected`
    /// webhooks): `Cancelled` means done; back to `Authorized` with
    /// `last_error_code`/`last_error_message` set means the Sefin rejected it.
    /// Repeating the request returns the current state.
    ///
    /// Invoices in `retry_pending`, `reconciliation_required` or
    /// `manual_review` fail with [`CorvoError::Api`] status 409, code
    /// `ERROR_INVOICE_NOT_CANCELLABLE`; an invalid description fails with 422.
    ///
    /// ```no_run
    /// # async fn run(corvo: corvo_nfse::Corvo) -> Result<(), corvo_nfse::CorvoError> {
    /// use corvo_nfse::{InvoiceCancelParams, InvoiceCancellationReason, InvoiceStatus};
    ///
    /// let params = InvoiceCancelParams::new(InvoiceCancellationReason::IssueError)
    ///     .description("Issued with the wrong amount");
    /// corvo.invoices().cancel("inv_123", &params).await?;
    ///
    /// let invoice = corvo.invoices().get("inv_123").await?;
    /// if invoice.status == InvoiceStatus::Cancelled {
    ///     println!("cancelled at {:?}", invoice.cancelled_at);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel(
        &self,
        id: &str,
        params: &InvoiceCancelParams,
    ) -> Result<Invoice, CorvoError> {
        let spec = RequestSpec::post(&format!("/invoices/{}/cancel", encode(id))).json(params)?;
        self.transport.request_json(&spec).await
    }
}

fn list_query(params: &InvoiceListParams) -> Vec<(String, String)> {
    let mut query = Vec::new();
    push_query(&mut query, "companyId", params.company_id.clone());
    push_query(
        &mut query,
        "status",
        params.status.map(|s| s.as_str().to_owned()),
    );
    push_query(&mut query, "from", params.from.clone());
    push_query(&mut query, "to", params.to.clone());
    push_query(&mut query, "externalId", params.external_id.clone());
    push_query(&mut query, "limit", params.limit.map(|l| l.to_string()));
    push_query(&mut query, "cursor", params.cursor.clone());
    query
}
