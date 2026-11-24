//! Schedule types. The wire format of schedules uses `snake_case`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::invoice::Frequency;

wire_enum! {
    /// Whether a schedule runs once or repeats.
    ScheduleKind {
        /// Runs once.
        Once => "once",
        /// Repeats.
        Recurring => "recurring",
    }
}

wire_enum! {
    /// State of a schedule.
    ScheduleStatus {
        /// Will run.
        Active => "active",
        /// Paused.
        Paused => "paused",
        /// Finished.
        Completed => "completed",
        /// Cancelled (final).
        Cancelled => "cancelled",
    }
}

wire_enum! {
    /// Why a schedule paused itself.
    PauseReason {
        /// The customer was the problem.
        Customer => "customer",
        /// Too many failures.
        Failures => "failures",
    }
}

wire_enum! {
    /// Outcome of one schedule run.
    ScheduleRunOutcome {
        /// An invoice was issued.
        Issued => "issued",
        /// The attempt failed.
        Failed => "failed",
    }
}

/// A scheduled or recurring invoice. The note only exists when the schedule
/// runs (09:00 America/Sao_Paulo, Monday to Friday).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    /// Unique id.
    pub id: String,
    /// Company that issues.
    pub company_id: String,
    /// `once` or `recurring`.
    pub kind: ScheduleKind,
    /// State.
    pub status: ScheduleStatus,
    /// First run (`YYYY-MM-DD`).
    pub anchor_on: String,
    /// Last day it can run.
    #[serde(default)]
    pub ends_on: Option<String>,
    /// How often it repeats, for a recurring schedule.
    #[serde(default)]
    pub frequency: Option<Frequency>,
    /// Day of the next run.
    #[serde(default)]
    pub next_run_on: Option<String>,
    /// Day of the last run.
    #[serde(default)]
    pub last_run_on: Option<String>,
    /// Failed attempts of the current occurrence.
    pub attempts: i64,
    /// Why it paused itself.
    #[serde(default)]
    pub pause_reason: Option<PauseReason>,
    /// The invoice body each run issues.
    pub invoice: Value,
    /// Creation timestamp.
    pub created_at: String,
}

/// One attempt to run a schedule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduleRun {
    /// Unique id.
    pub id: String,
    /// Attempt number.
    pub attempt: i64,
    /// `issued` or `failed`.
    pub outcome: ScheduleRunOutcome,
    /// Day the occurrence was due.
    pub occurrence_on: String,
    /// Day it ran.
    pub ran_on: String,
    /// Invoice issued, when it worked.
    #[serde(default)]
    pub invoice_id: Option<String>,
    /// Error code, when it failed.
    #[serde(default)]
    pub error_code: Option<String>,
    /// Error message, when it failed.
    #[serde(default)]
    pub error_message: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
}

/// Filters for [`Schedules::list`](crate::Schedules::list). The cursor is the
/// offset of the next page: pass it back untouched.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScheduleListParams {
    /// Only schedules in this state.
    pub status: Option<ScheduleStatus>,
    /// Only schedules of this company.
    pub company_id: Option<String>,
    /// Page size (1–1000, default 50).
    pub limit: Option<u16>,
    /// Opaque cursor from a previous page.
    pub cursor: Option<String>,
}

/// Change to a schedule; applies to the next runs only.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleUpdateParams {
    /// New first/next run day (`YYYY-MM-DD`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_on: Option<String>,
    /// New last day; `Some(None)` removes the end.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ends_on: Option<Option<String>>,
    /// New frequency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    /// Invoice body fields to change, merged over the stored body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Value>,
}
