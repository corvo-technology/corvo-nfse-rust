//! Scheduled and recurring invoices: 7 ops.

use crate::error::CorvoError;
use crate::resources::companies::encode;
use crate::transport::{push_query, Paginator, RequestSpec, Unwrap};
use crate::types::common::Paginated;
use crate::types::schedule::{Schedule, ScheduleListParams, ScheduleRun, ScheduleUpdateParams};

/// Schedules resource, obtained via [`Corvo::schedules()`](crate::Corvo::schedules).
///
/// Create a schedule with [`Invoices::create`](crate::Invoices::create) and
/// `options.type` `SCHEDULE` or `RECURRING`.
pub struct Schedules<'c> {
    pub(crate) transport: &'c crate::transport::Transport,
}

impl<'c> Schedules<'c> {
    pub(crate) fn new(transport: &'c crate::transport::Transport) -> Self {
        Self { transport }
    }

    /// List scheduled and recurring invoices (`GET /schedules`).
    pub async fn list(
        &self,
        params: &ScheduleListParams,
    ) -> Result<Paginated<Schedule>, CorvoError> {
        self.transport
            .list_page("/schedules", &list_query(params), Unwrap::None)
            .await
    }

    /// Stream every schedule across pages (`GET /schedules`).
    pub fn iterate(&self, params: &ScheduleListParams) -> Paginator<'c, Schedule> {
        Paginator::new(
            self.transport,
            "/schedules",
            list_query(params),
            Unwrap::None,
            params.cursor.clone(),
        )
    }

    /// Get a schedule (`GET /schedules/:id`).
    pub async fn get(&self, id: &str) -> Result<Schedule, CorvoError> {
        let spec = RequestSpec::get(&format!("/schedules/{}", encode(id)));
        self.transport.request_json(&spec).await
    }

    /// Change a schedule; applies to the next runs only (`PATCH /schedules/:id`).
    pub async fn update(
        &self,
        id: &str,
        params: &ScheduleUpdateParams,
    ) -> Result<Schedule, CorvoError> {
        let spec = RequestSpec::patch(&format!("/schedules/{}", encode(id))).json(params)?;
        self.transport.request_json(&spec).await
    }

    /// Pause a schedule (`POST /schedules/:id/pause`).
    pub async fn pause(&self, id: &str) -> Result<Schedule, CorvoError> {
        let spec = RequestSpec::post(&format!("/schedules/{}/pause", encode(id)));
        self.transport.request_json(&spec).await
    }

    /// Resume a paused schedule at its next occurrence
    /// (`POST /schedules/:id/resume`).
    pub async fn resume(&self, id: &str) -> Result<Schedule, CorvoError> {
        let spec = RequestSpec::post(&format!("/schedules/{}/resume", encode(id)));
        self.transport.request_json(&spec).await
    }

    /// Cancel a schedule; this is final (`POST /schedules/:id/cancel`).
    pub async fn cancel(&self, id: &str) -> Result<Schedule, CorvoError> {
        let spec = RequestSpec::post(&format!("/schedules/{}/cancel", encode(id)));
        self.transport.request_json(&spec).await
    }

    /// The log of every attempt to run a schedule, newest first
    /// (`GET /schedules/:id/runs`).
    pub async fn list_runs(
        &self,
        id: &str,
        limit: Option<u16>,
        cursor: Option<&str>,
    ) -> Result<Paginated<ScheduleRun>, CorvoError> {
        self.transport
            .list_page(
                &format!("/schedules/{}/runs", encode(id)),
                &page_query(limit, cursor),
                Unwrap::None,
            )
            .await
    }
}

fn page_query(limit: Option<u16>, cursor: Option<&str>) -> Vec<(String, String)> {
    let mut query = Vec::new();
    push_query(&mut query, "limit", limit.map(|l| l.to_string()));
    push_query(&mut query, "cursor", cursor.map(str::to_owned));
    query
}

fn list_query(params: &ScheduleListParams) -> Vec<(String, String)> {
    let mut query = Vec::new();
    push_query(
        &mut query,
        "status",
        params.status.map(|s| s.as_str().to_owned()),
    );
    push_query(&mut query, "companyId", params.company_id.clone());
    push_query(&mut query, "limit", params.limit.map(|l| l.to_string()));
    push_query(&mut query, "cursor", params.cursor.clone());
    query
}
