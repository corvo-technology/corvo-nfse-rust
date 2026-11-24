//! Resource namespaces composed over the shared transport.

pub mod accountant_package;
pub mod billing;
pub mod certificates;
pub mod companies;
pub mod credits;
pub mod customers;
pub mod dps_sequence;
pub mod fiscal_portal;
pub mod fiscal_profiles;
pub mod health;
pub mod imports;
pub mod integrations;
pub mod invoice_package;
pub mod invoices;
pub mod plans;
pub mod schedules;
pub mod service_favorites;
pub mod webhooks;

pub use accountant_package::AccountantPackage;
pub use billing::Billing;
pub use certificates::Certificates;
pub use companies::{Companies, CompanyListParams};
pub use credits::{CreditPlanListParams, Credits};
pub use customers::{Cnpj, Customers};
pub use dps_sequence::DpsSequenceResource;
pub use fiscal_portal::FiscalPortal;
pub use fiscal_profiles::{FiscalProfiles, ServiceTaxProfiles};
pub use health::Health;
pub use imports::Imports;
pub use integrations::{IntegrationConnections, IntegrationOperations, Integrations};
pub use invoice_package::InvoicePackage;
pub use invoices::Invoices;
pub use plans::Plans;
pub use schedules::Schedules;
pub use service_favorites::ServiceFavorites;
pub use webhooks::{Deliveries, WebhookListParams, Webhooks};
