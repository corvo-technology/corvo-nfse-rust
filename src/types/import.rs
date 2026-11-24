//! Spreadsheet import types. The wire format of imports uses `snake_case`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

wire_enum! {
    /// State of an import.
    ImportStatus {
        /// The file is being read.
        Processing => "processing",
        /// Read; waiting for `emit`.
        Ready => "ready",
        /// Invoices are being issued.
        Emitting => "emitting",
        /// Finished.
        Done => "done",
        /// The file could not be used.
        Failed => "failed",
    }
}

wire_enum! {
    /// State of one row of an import.
    ImportRowStatus {
        /// Checked and ready to emit.
        Valid => "valid",
        /// Needs a correction.
        Invalid => "invalid",
        /// Waiting to be issued.
        Queued => "queued",
        /// The invoice was accepted.
        Accepted => "accepted",
        /// The invoice was rejected.
        Rejected => "rejected",
    }
}

/// Counters of an import.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportCounters {
    /// Rows read.
    pub total: i64,
    /// Valid rows.
    pub valid: i64,
    /// Invalid rows.
    pub invalid: i64,
    /// Rows accepted by the API.
    pub accepted: i64,
    /// Rows still pending.
    pub pending: i64,
    /// Invoices authorized.
    pub authorized: i64,
    /// Rows that failed.
    pub failed: i64,
    /// Credits missing to emit everything.
    pub awaiting_credits: i64,
    /// Amount of the authorized rows.
    pub authorized_amount: f64,
    /// Amount waiting for credits.
    pub awaiting_amount: f64,
    /// Amount of the failed rows.
    pub failed_amount: f64,
    /// Amount of the pending rows.
    pub pending_amount: f64,
    /// Amount of the valid rows.
    pub valid_amount: f64,
}

/// An import and its counters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportJob {
    /// Unique id.
    pub id: String,
    /// Company the invoices are issued from, when one was chosen.
    #[serde(default)]
    pub company_id: Option<String>,
    /// Counters.
    pub counters: ImportCounters,
    /// Creation timestamp.
    pub created_at: String,
    /// When emitting started.
    #[serde(default)]
    pub emit_started_at: Option<String>,
    /// When emitting finished.
    #[serde(default)]
    pub emit_finished_at: Option<String>,
    /// Error code, when `status` is `failed`.
    #[serde(default)]
    pub error_code: Option<String>,
    /// Name of the uploaded file.
    pub file_name: String,
    /// Rows the file announced.
    #[serde(default)]
    pub rows_expected: Option<i64>,
    /// State.
    pub status: ImportStatus,
    /// Columns of the file that Corvo does not know.
    #[serde(default)]
    pub unknown_columns: Vec<String>,
}

/// One row of an import.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportRow {
    /// Line in the file (starts at 1).
    pub line: i64,
    /// State.
    pub status: ImportRowStatus,
    /// The row as an invoice body (free-form).
    pub data: Value,
    /// Error code, for an invalid or rejected row.
    #[serde(default)]
    pub error_code: Option<String>,
    /// Error message, for an invalid or rejected row.
    #[serde(default)]
    pub error_message: Option<String>,
    /// Invoice created from the row.
    #[serde(default)]
    pub invoice_id: Option<String>,
}

/// Answer of [`Imports::start`](crate::Imports::start).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportStart {
    /// Unique id.
    pub id: String,
    /// State.
    pub status: ImportStatus,
    /// Id of the earlier import of the same file, when it is a duplicate.
    #[serde(default)]
    pub duplicate_of: Option<String>,
}

/// File to import: CSV or Excel. Sent as `multipart/form-data`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportStartParams {
    /// Raw bytes of the spreadsheet.
    pub file: Vec<u8>,
    /// File name reported to the server; defaults to `import.csv`.
    pub file_name: Option<String>,
    /// Company to issue from (when the spreadsheet has no company column).
    pub company_id: Option<String>,
}

impl ImportStartParams {
    /// Import the spreadsheet `file`.
    pub fn new(file: impl Into<Vec<u8>>) -> Self {
        Self {
            file: file.into(),
            file_name: None,
            company_id: None,
        }
    }

    /// Set the reported file name.
    pub fn file_name(mut self, name: impl Into<String>) -> Self {
        self.file_name = Some(name.into());
        self
    }

    /// Set the company to issue from.
    pub fn company_id(mut self, id: impl Into<String>) -> Self {
        self.company_id = Some(id.into());
        self
    }
}

/// Body of [`Imports::emit`](crate::Imports::emit).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportEmitParams {
    /// Whether to e-mail each invoice to its customer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_email: Option<bool>,
}

/// Filters for [`Imports::rows`](crate::Imports::rows). The cursor is the
/// offset of the next page: pass it back untouched.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportRowListParams {
    /// Only rows in this state.
    pub status: Option<ImportRowStatus>,
    /// Page size (1–200, default 50).
    pub limit: Option<u16>,
    /// Opaque cursor from a previous page.
    pub cursor: Option<String>,
}
