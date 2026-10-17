//! Resource namespaces composed over the shared transport.

pub mod billing;
pub mod certificates;
pub mod companies;
pub mod credits;
pub mod health;
pub mod invoices;
pub mod plans;
pub mod webhooks;

pub use billing::Billing;
pub use certificates::Certificates;
pub use companies::{Companies, CompanyListParams};
pub use credits::{CreditPlanListParams, Credits};
pub use health::Health;
pub use invoices::Invoices;
pub use plans::Plans;
pub use webhooks::{Deliveries, WebhookListParams, Webhooks};
