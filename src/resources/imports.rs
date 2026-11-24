//! Spreadsheet imports: 9 ops.

use reqwest::multipart::{Form, Part};
use serde_json::json;

use crate::error::CorvoError;
use crate::resources::companies::encode;
use crate::transport::{push_query, DownloadedFile, Paginator, RequestSpec, Unwrap};
use crate::types::common::Paginated;
use crate::types::import::{
    ImportEmitParams, ImportJob, ImportRow, ImportRowListParams, ImportStart, ImportStartParams,
};

/// Imports resource, obtained via [`Corvo::imports()`](crate::Corvo::imports).
pub struct Imports<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> Imports<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// Import invoices from a CSV or Excel spreadsheet (`POST /imports`).
    ///
    /// The multipart body is built for you. The import is read in the
    /// background: follow it with [`get`](Self::get) until `status` is `ready`,
    /// then call [`emit`](Self::emit).
    pub async fn start(&self, params: &ImportStartParams) -> Result<ImportStart, CorvoError> {
        let file_name = params
            .file_name
            .clone()
            .unwrap_or_else(|| "import.csv".to_owned());
        let file_bytes = params.file.clone();
        let company_id = params.company_id.clone();
        let mime = mime_for(&file_name);
        let spec = RequestSpec::post("/imports").form(move || {
            let part = Part::bytes(file_bytes.clone())
                .file_name(file_name.clone())
                .mime_str(mime)
                .expect("valid spreadsheet mime");
            let mut form = Form::new();
            if let Some(id) = &company_id {
                form = form.text("companyId", id.clone());
            }
            form.part("file", part)
        });
        self.transport.request_json(&spec).await
    }

    /// Recent imports with their counters, newest first, at most 20
    /// (`GET /imports`).
    pub async fn list(&self, limit: Option<u16>) -> Result<Vec<ImportJob>, CorvoError> {
        let mut query = Vec::new();
        push_query(&mut query, "limit", limit.map(|l| l.to_string()));
        let spec = RequestSpec::get("/imports").query(query);
        self.transport.request_json(&spec).await
    }

    /// Status and counters of an import (`GET /imports/:id`).
    pub async fn get(&self, id: &str) -> Result<ImportJob, CorvoError> {
        let spec = RequestSpec::get(&format!("/imports/{}", encode(id)));
        self.transport.request_json(&spec).await
    }

    /// Emit every valid row of the import (`POST /imports/:id/emit`).
    pub async fn emit(&self, id: &str, params: &ImportEmitParams) -> Result<ImportJob, CorvoError> {
        let mut spec = RequestSpec::post(&format!("/imports/{}/emit", encode(id)));
        if params.send_email.is_some() {
            spec = spec.json(params)?;
        }
        self.transport.request_json(&spec).await
    }

    /// Rows of an import, one page at a time (`GET /imports/:id/rows`).
    pub async fn rows(
        &self,
        id: &str,
        params: &ImportRowListParams,
    ) -> Result<Paginated<ImportRow>, CorvoError> {
        self.transport
            .list_page(
                &format!("/imports/{}/rows", encode(id)),
                &rows_query(params),
                Unwrap::None,
            )
            .await
    }

    /// Stream every row of an import across pages (`GET /imports/:id/rows`).
    pub fn iterate_rows(&self, id: &str, params: &ImportRowListParams) -> Paginator<'c, ImportRow> {
        Paginator::new(
            self.transport,
            &format!("/imports/{}/rows", encode(id)),
            rows_query(params),
            Unwrap::None,
            params.cursor.clone(),
        )
    }

    /// Correct one row and check it again; only while the import is `ready`
    /// (`PATCH /imports/:id/rows/:line`).
    ///
    /// `item` holds the invoice fields to change (same shape as
    /// `POST /invoices`).
    pub async fn update_row(
        &self,
        id: &str,
        line: u32,
        item: &serde_json::Value,
    ) -> Result<ImportRow, CorvoError> {
        let spec = RequestSpec::patch(&format!("/imports/{}/rows/{}", encode(id), line))
            .json(&json!({ "item": item }))?;
        self.transport.request_json(&spec).await
    }

    /// Discard an import that has not been emitted (`DELETE /imports/:id`).
    pub async fn discard(&self, id: &str) -> Result<(), CorvoError> {
        let spec = RequestSpec::delete(&format!("/imports/{}", encode(id)));
        self.transport.request_unit(&spec).await
    }

    /// Download the spreadsheet template, CSV (`GET /imports/template`).
    pub async fn download_template(&self) -> Result<DownloadedFile, CorvoError> {
        let spec = RequestSpec::get("/imports/template").accept("text/csv");
        self.transport.request_raw(&spec).await
    }

    /// Download the CSV with the rows that need attention
    /// (`GET /imports/:id/errors.csv`).
    pub async fn download_errors(&self, id: &str) -> Result<DownloadedFile, CorvoError> {
        let spec =
            RequestSpec::get(&format!("/imports/{}/errors.csv", encode(id))).accept("text/csv");
        self.transport.request_raw(&spec).await
    }
}

fn rows_query(params: &ImportRowListParams) -> Vec<(String, String)> {
    let mut query = Vec::new();
    push_query(
        &mut query,
        "status",
        params.status.map(|s| s.as_str().to_owned()),
    );
    push_query(&mut query, "limit", params.limit.map(|l| l.to_string()));
    push_query(&mut query, "cursor", params.cursor.clone());
    query
}

fn mime_for(file_name: &str) -> &'static str {
    let lower = file_name.to_ascii_lowercase();
    if lower.ends_with(".xlsx") {
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
    } else if lower.ends_with(".xls") {
        "application/vnd.ms-excel"
    } else {
        "text/csv"
    }
}
