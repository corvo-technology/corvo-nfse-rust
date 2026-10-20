//! Invoice cancellation: `invoices().cancel()`, the `cancelling`/`cancelled`
//! statuses, and the cancellation fields on `Invoice`.

use corvo_nfse::{
    Corvo, CorvoError, InvoiceCancelParams, InvoiceCancellationReason, InvoiceStatus,
    WEBHOOK_EVENTS,
};
use pretty_assertions::assert_eq;
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const API_KEY: &str = "corvo_test_0123456789abcdef0123456789abcdef01234567";

fn client_for(server: &MockServer) -> Corvo {
    Corvo::builder()
        .api_key(API_KEY)
        .base_url(server.uri())
        .build()
        .expect("test client")
}

/// An invoice as the API returns it, with the given status; callers override
/// the cancellation fields.
fn invoice_json(id: &str, status: &str) -> serde_json::Value {
    json!({
        "id": id,
        "companyId": "c_1",
        "tenantId": "ten_1",
        "environment": "test",
        "documentKeyVersion": 1,
        "emailSnapshot": {},
        "externalId": null,
        "idempotencyKey": null,
        "status": status,
        "customerSnapshot": { "name": "Customer", "taxId": "52998224725" },
        "serviceSnapshot": { "code": "010101" },
        "taxSnapshot": {},
        "amount": 10.5,
        "currency": "BRL",
        "competenceDate": "2026-10-13",
        "dpsId": null, "dpsSeries": null, "dpsNumber": null,
        "accessKey": null, "nfseNumber": null,
        "fiscalProvider": null, "fiscalSchemaVersion": null,
        "issuedAt": null, "authorizedAt": null,
        "lastErrorCode": null, "lastErrorMessage": null, "lastErrorProvider": null,
        "documentsReadyAt": null,
        "lastEmailStatus": null, "lastEmailSentAt": null,
        "createdAt": "2026-10-13T00:00:00Z",
        "updatedAt": "2026-10-13T00:00:00Z",
        "documentsStatus": "pending",
        "emailStatus": null,
        "stage": "pending",
        "scheduledFor": null,
        "cancellationReason": null,
        "cancellationDescription": null,
        "cancelledAt": null,
        "cancellationRequestedAt": null
    })
}

#[tokio::test]
async fn cancel_posts_reason_and_description_and_returns_the_cancelled_invoice() {
    let server = MockServer::start().await;
    let mut cancelled = invoice_json("inv_1", "cancelled");
    cancelled["cancellationReason"] = json!("issue_error");
    cancelled["cancellationDescription"] = json!("Wrong amount on the invoice");
    cancelled["cancelledAt"] = json!("2026-10-20T12:00:00Z");
    Mock::given(method("POST"))
        .and(path("/v1/invoices/inv_1/cancel"))
        .and(header("x-api-key", API_KEY))
        .and(body_json(json!({
            "reason": "issue_error",
            "description": "Wrong amount on the invoice"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "data": cancelled })))
        .expect(1)
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let params = InvoiceCancelParams {
        reason: InvoiceCancellationReason::IssueError,
        description: Some("Wrong amount on the invoice".into()),
    };
    let invoice = corvo
        .invoices()
        .cancel("inv_1", &params)
        .await
        .expect("cancel");

    assert_eq!(invoice.id, "inv_1");
    assert_eq!(invoice.status, InvoiceStatus::Cancelled);
    assert_eq!(
        invoice.cancellation_reason,
        Some(InvoiceCancellationReason::IssueError)
    );
    assert_eq!(
        invoice.cancellation_description.as_deref(),
        Some("Wrong amount on the invoice")
    );
    assert_eq!(
        invoice.cancelled_at.as_deref(),
        Some("2026-10-20T12:00:00Z")
    );
}

#[tokio::test]
async fn cancel_omits_a_missing_description() {
    let server = MockServer::start().await;
    let mut cancelled = invoice_json("inv_1", "cancelled");
    cancelled["cancellationReason"] = json!("other");
    Mock::given(method("POST"))
        .and(path("/v1/invoices/inv_1/cancel"))
        .and(body_json(json!({ "reason": "other" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "data": cancelled })))
        .expect(1)
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let params = InvoiceCancelParams::new(InvoiceCancellationReason::Other);
    let invoice = corvo
        .invoices()
        .cancel("inv_1", &params)
        .await
        .expect("cancel");
    assert_eq!(
        invoice.cancellation_reason,
        Some(InvoiceCancellationReason::Other)
    );
}

#[tokio::test]
async fn cancel_202_returns_the_cancelling_invoice() {
    let server = MockServer::start().await;
    let mut cancelling = invoice_json("inv_2", "cancelling");
    cancelling["cancellationReason"] = json!("service_not_provided");
    cancelling["cancellationDescription"] = json!("The service was never provided");
    cancelling["cancellationRequestedAt"] = json!("2026-10-20T12:00:00Z");
    Mock::given(method("POST"))
        .and(path("/v1/invoices/inv_2/cancel"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({ "data": cancelling })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let params = InvoiceCancelParams::new(InvoiceCancellationReason::ServiceNotProvided)
        .description("The service was never provided");
    let invoice = corvo
        .invoices()
        .cancel("inv_2", &params)
        .await
        .expect("cancel");
    assert_eq!(invoice.status, InvoiceStatus::Cancelling);
    assert_eq!(
        invoice.cancellation_reason,
        Some(InvoiceCancellationReason::ServiceNotProvided)
    );
    assert_eq!(invoice.cancelled_at, None);
    assert_eq!(
        invoice.cancellation_requested_at.as_deref(),
        Some("2026-10-20T12:00:00Z")
    );
}

#[tokio::test]
async fn cancel_202_on_a_processing_invoice_keeps_processing() {
    let server = MockServer::start().await;
    let mut processing = invoice_json("inv_3", "processing");
    processing["cancellationReason"] = json!("issue_error");
    processing["cancellationDescription"] = json!("Issued to the wrong customer");
    processing["cancellationRequestedAt"] = json!("2026-10-20T12:00:00Z");
    Mock::given(method("POST"))
        .and(path("/v1/invoices/inv_3/cancel"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({ "data": processing })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let params = InvoiceCancelParams::new(InvoiceCancellationReason::IssueError)
        .description("Issued to the wrong customer");
    let invoice = corvo
        .invoices()
        .cancel("inv_3", &params)
        .await
        .expect("cancel");
    assert_eq!(invoice.status, InvoiceStatus::Processing);
    assert!(invoice.cancellation_requested_at.is_some());
}

#[tokio::test]
async fn cancel_409_surfaces_as_api_error_with_the_code() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/invoices/inv_4/cancel"))
        .respond_with(ResponseTemplate::new(409).set_body_json(json!({
            "code": "ERROR_INVOICE_NOT_CANCELLABLE",
            "message": "Invoice cannot be cancelled in status manual_review",
            "requestId": "req_409"
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let err = corvo
        .invoices()
        .cancel(
            "inv_4",
            &InvoiceCancelParams::new(InvoiceCancellationReason::Other),
        )
        .await
        .expect_err("not cancellable");
    assert!(
        matches!(err, CorvoError::Api { status: 409, .. }),
        "{err:?}"
    );
    assert_eq!(err.status(), 409);
    assert_eq!(err.code(), "ERROR_INVOICE_NOT_CANCELLABLE");
    assert_eq!(err.request_id(), Some("req_409"));
}

#[tokio::test]
async fn cancel_422_surfaces_as_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/invoices/inv_5/cancel"))
        .respond_with(ResponseTemplate::new(422).set_body_json(json!({
            "code": "ERROR_VALIDATION",
            "message": "Validation failed",
            "details": [{ "field": "description", "code": "too_short", "message": "min 15" }]
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let err = corvo
        .invoices()
        .cancel(
            "inv_5",
            &InvoiceCancelParams::new(InvoiceCancellationReason::Other).description("short"),
        )
        .await
        .expect_err("invalid description");
    assert_eq!(err.status(), 422);
    assert_eq!(err.code(), "ERROR_VALIDATION");
    match err {
        CorvoError::Api { details, .. } => assert_eq!(details[0].field, "description"),
        other => panic!("expected Api error, got {other:?}"),
    }
}

#[tokio::test]
async fn get_parses_a_cancelled_invoice() {
    let server = MockServer::start().await;
    let mut cancelled = invoice_json("inv_6", "cancelled");
    cancelled["cancellationReason"] = json!("service_not_provided");
    cancelled["cancellationDescription"] = json!("The service was never provided");
    cancelled["cancelledAt"] = json!("2026-10-20T12:05:00Z");
    cancelled["cancellationRequestedAt"] = json!("2026-10-20T12:00:00Z");
    Mock::given(method("GET"))
        .and(path("/v1/invoices/inv_6"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "data": cancelled })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let invoice = corvo.invoices().get("inv_6").await.expect("get");
    assert_eq!(invoice.status, InvoiceStatus::Cancelled);
    assert_eq!(invoice.status.as_str(), "cancelled");
    assert_eq!(
        invoice.cancellation_reason,
        Some(InvoiceCancellationReason::ServiceNotProvided)
    );
    assert_eq!(
        invoice.cancellation_description.as_deref(),
        Some("The service was never provided")
    );
    assert_eq!(
        invoice.cancelled_at.as_deref(),
        Some("2026-10-20T12:05:00Z")
    );
    assert_eq!(
        invoice.cancellation_requested_at.as_deref(),
        Some("2026-10-20T12:00:00Z")
    );
}

#[tokio::test]
async fn get_parses_a_cancelling_invoice() {
    let server = MockServer::start().await;
    let mut cancelling = invoice_json("inv_7", "cancelling");
    cancelling["cancellationReason"] = json!("other");
    cancelling["cancellationRequestedAt"] = json!("2026-10-20T12:00:00Z");
    Mock::given(method("GET"))
        .and(path("/v1/invoices/inv_7"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "data": cancelling })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let invoice = corvo.invoices().get("inv_7").await.expect("get");
    assert_eq!(invoice.status, InvoiceStatus::Cancelling);
    assert_eq!(invoice.status.as_str(), "cancelling");
    assert_eq!(
        invoice.cancellation_reason,
        Some(InvoiceCancellationReason::Other)
    );
    assert_eq!(invoice.cancellation_description, None);
    assert_eq!(invoice.cancelled_at, None);
}

#[tokio::test]
async fn get_parses_an_invoice_without_the_cancellation_fields() {
    // Older payloads (or invoices never cancelled) may omit the fields entirely.
    let server = MockServer::start().await;
    let mut authorized = invoice_json("inv_8", "authorized");
    let obj = authorized.as_object_mut().unwrap();
    for key in [
        "cancellationReason",
        "cancellationDescription",
        "cancelledAt",
        "cancellationRequestedAt",
    ] {
        obj.remove(key);
    }
    Mock::given(method("GET"))
        .and(path("/v1/invoices/inv_8"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "data": authorized })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let invoice = corvo.invoices().get("inv_8").await.expect("get");
    assert_eq!(invoice.status, InvoiceStatus::Authorized);
    assert_eq!(invoice.cancellation_reason, None);
    assert_eq!(invoice.cancelled_at, None);
}

#[test]
fn cancellation_reason_wire_values() {
    assert_eq!(
        InvoiceCancellationReason::IssueError.as_str(),
        "issue_error"
    );
    assert_eq!(
        InvoiceCancellationReason::ServiceNotProvided.as_str(),
        "service_not_provided"
    );
    assert_eq!(InvoiceCancellationReason::Other.as_str(), "other");
}

#[test]
fn webhook_events_include_the_cancellation_events() {
    assert!(WEBHOOK_EVENTS.contains(&"nfse.invoice.cancelled"));
    assert!(WEBHOOK_EVENTS.contains(&"nfse.invoice.cancellation_rejected"));
}
