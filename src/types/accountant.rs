//! Accountant package types.

use serde::{Deserialize, Serialize};

/// Accountant e-mail of the monthly package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountantSettings {
    /// Where the package goes; `None` means the monthly send is off.
    pub email: Option<String>,
    /// Last month sent (`YYYY-MM`).
    pub last_sent_month: Option<String>,
}

/// Result of [`AccountantPackage::update`](crate::AccountantPackage::update).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountantSettingsUpdate {
    /// Where the package goes; `None` means the monthly send is off.
    pub email: Option<String>,
    /// Last month sent (`YYYY-MM`).
    pub last_sent_month: Option<String>,
    /// Whether a test e-mail was sent to the new address.
    pub test_email_sent: bool,
}

/// Body of [`AccountantPackage::update`](crate::AccountantPackage::update).
/// `accountant_email: None` is sent as `null` and turns the monthly send off.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountantPackageUpdateParams {
    /// The accountant e-mail, or `None` to turn the monthly send off.
    pub accountant_email: Option<String>,
}

wire_enum! {
    /// Outcome of [`AccountantPackage::send`](crate::AccountantPackage::send).
    AccountantSendOutcome {
        /// Sent.
        Sent => "sent",
        /// Not sent: a test key.
        SkippedTest => "skipped-test",
        /// Not sent: no accountant e-mail set.
        SkippedNoEmail => "skipped-no-email",
        /// Not sent: no invoices in the month.
        SkippedEmpty => "skipped-empty",
        /// Not sent: the package is too large.
        SkippedTooLarge => "skipped-too-large",
        /// Not sent: the month was already sent.
        SkippedAlreadySent => "skipped-already-sent",
    }
}

/// Result of sending the package now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountantSendResult {
    /// What happened.
    pub status: AccountantSendOutcome,
}

/// One attempt to send the package to the accountant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountantSend {
    /// Unique id.
    pub id: String,
    /// When it was tried (RFC 3339).
    pub created_at: String,
    /// Invoices in the package.
    pub invoice_count: i64,
    /// Month (`YYYY-MM`).
    pub month: String,
    /// E-mail it was sent to.
    pub recipient: String,
    /// `sent`, `skipped-empty`, `skipped-too-large` or `failed`.
    pub status: String,
    /// Total of the package in centavos.
    pub total_cents: i64,
    /// `scheduled`, `manual` or `mcp-or-api`.
    pub trigger: String,
}
