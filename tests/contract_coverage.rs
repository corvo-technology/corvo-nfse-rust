//! Contract coverage: every operationId of the vendored `spec/openapi.json` must
//! be implemented by the SDK. When the API contract changes, replace the spec
//! file; this test fails until the new operations are in `IMPLEMENTED`.
//!
//! Each row is `(operationId, HTTP method, path, SDK method)`. The test checks
//! the method and path against the spec, and that the SDK method exists in
//! `src/resources/`.

use std::collections::BTreeSet;
use std::fs;

use serde_json::Value;

/// (operationId, method, path, `resource.fn`)
const IMPLEMENTED: &[(&str, &str, &str, &str)] = &[
    (
        "accountantPackage_get",
        "GET",
        "/companies/{companyId}/accountant-package",
        "accountant_package.get",
    ),
    (
        "accountantPackage_listSends",
        "GET",
        "/companies/{companyId}/accountant-package/sends",
        "accountant_package.list_sends",
    ),
    (
        "accountantPackage_send",
        "POST",
        "/companies/{companyId}/accountant-package/send",
        "accountant_package.send",
    ),
    (
        "accountantPackage_update",
        "PUT",
        "/companies/{companyId}/accountant-package",
        "accountant_package.update",
    ),
    ("billing_getPlan", "GET", "/billing/plan", "billing.plan"),
    ("billing_getUsage", "GET", "/billing/usage", "billing.usage"),
    (
        "certificates_list",
        "GET",
        "/companies/{companyId}/certificates",
        "certificates.list",
    ),
    (
        "certificates_revoke",
        "DELETE",
        "/companies/{companyId}/certificates/{certId}",
        "certificates.remove",
    ),
    (
        "certificates_upload",
        "POST",
        "/companies/{companyId}/certificates",
        "certificates.upload",
    ),
    ("cnpj_lookup", "GET", "/cnpj/{cnpj}", "cnpj.lookup"),
    ("companies_create", "POST", "/companies", "companies.create"),
    (
        "companies_delete",
        "DELETE",
        "/companies/{id}",
        "companies.remove",
    ),
    ("companies_get", "GET", "/companies/{id}", "companies.get"),
    ("companies_list", "GET", "/companies", "companies.list"),
    (
        "companies_update",
        "PATCH",
        "/companies/{id}",
        "companies.update",
    ),
    ("credits_getBalance", "GET", "/credits", "credits.balance"),
    (
        "credits_listHistory",
        "GET",
        "/credits/history",
        "credits.history",
    ),
    (
        "credits_listPlans",
        "GET",
        "/credits/plans",
        "credits.plans",
    ),
    (
        "credits_previewPurchase",
        "GET",
        "/credits/purchase/preview",
        "credits.preview_purchase",
    ),
    (
        "customers_delete",
        "DELETE",
        "/customers/{taxId}",
        "customers.remove",
    ),
    (
        "customers_get",
        "GET",
        "/customers/{taxId}",
        "customers.get",
    ),
    ("customers_search", "GET", "/customers", "customers.search"),
    (
        "customers_update",
        "PATCH",
        "/customers/{taxId}",
        "customers.update",
    ),
    (
        "dpsSequence_get",
        "GET",
        "/companies/{id}/dps-sequence",
        "dps_sequence.get",
    ),
    (
        "dpsSequence_set",
        "PUT",
        "/companies/{id}/dps-sequence",
        "dps_sequence.set",
    ),
    (
        "fiscalPortal_status",
        "GET",
        "/fiscal-portal/status",
        "fiscal_portal.status",
    ),
    (
        "fiscalProfiles_approve",
        "POST",
        "/companies/{id}/fiscal-profile/{profileId}/approve",
        "fiscal_profiles.approve",
    ),
    (
        "fiscalProfiles_list",
        "GET",
        "/companies/{id}/fiscal-profile",
        "fiscal_profiles.list",
    ),
    (
        "fiscalProfiles_upsert",
        "PUT",
        "/companies/{id}/fiscal-profile",
        "fiscal_profiles.upsert",
    ),
    ("health_live", "GET", "/health/live", "health.live"),
    (
        "imports_discard",
        "DELETE",
        "/imports/{id}",
        "imports.discard",
    ),
    (
        "imports_downloadErrors",
        "GET",
        "/imports/{id}/errors.csv",
        "imports.download_errors",
    ),
    (
        "imports_downloadTemplate",
        "GET",
        "/imports/template",
        "imports.download_template",
    ),
    ("imports_emit", "POST", "/imports/{id}/emit", "imports.emit"),
    ("imports_get", "GET", "/imports/{id}", "imports.get"),
    ("imports_list", "GET", "/imports", "imports.list"),
    (
        "imports_listRows",
        "GET",
        "/imports/{id}/rows",
        "imports.rows",
    ),
    ("imports_start", "POST", "/imports", "imports.start"),
    (
        "imports_updateRow",
        "PATCH",
        "/imports/{id}/rows/{line}",
        "imports.update_row",
    ),
    (
        "integrationConnections_activate",
        "POST",
        "/integrations/connections/{id}/activate",
        "integration_connections.activate",
    ),
    (
        "integrationConnections_create",
        "POST",
        "/integrations/connections",
        "integration_connections.create",
    ),
    (
        "integrationConnections_disconnect",
        "POST",
        "/integrations/connections/{id}/disconnect",
        "integration_connections.disconnect",
    ),
    (
        "integrationConnections_get",
        "GET",
        "/integrations/connections/{id}",
        "integration_connections.get",
    ),
    (
        "integrationConnections_list",
        "GET",
        "/integrations/connections",
        "integration_connections.list",
    ),
    (
        "integrationConnections_listEvents",
        "GET",
        "/integrations/connections/{id}/events",
        "integration_connections.list_events",
    ),
    (
        "integrationConnections_listOperations",
        "GET",
        "/integrations/connections/{id}/operations",
        "integration_connections.list_operations",
    ),
    (
        "integrationConnections_listRules",
        "GET",
        "/integrations/connections/{id}/rules",
        "integration_connections.list_rules",
    ),
    (
        "integrationConnections_pause",
        "POST",
        "/integrations/connections/{id}/pause",
        "integration_connections.pause",
    ),
    (
        "integrationConnections_replaceRules",
        "PUT",
        "/integrations/connections/{id}/rules",
        "integration_connections.replace_rules",
    ),
    (
        "integrationConnections_updateCredentials",
        "PUT",
        "/integrations/connections/{id}/credentials",
        "integration_connections.update_credentials",
    ),
    (
        "integrationConnections_validate",
        "POST",
        "/integrations/connections/{id}/validate",
        "integration_connections.validate",
    ),
    (
        "integrationOperations_completeCustomer",
        "PATCH",
        "/integrations/operations/{id}/customer",
        "integration_operations.complete_customer",
    ),
    (
        "integrationOperations_retry",
        "POST",
        "/integrations/operations/{id}/retry",
        "integration_operations.retry",
    ),
    (
        "integrations_listProviders",
        "GET",
        "/integrations/providers",
        "integrations.list_providers",
    ),
    (
        "invoicePackage_download",
        "GET",
        "/companies/{companyId}/invoice-package",
        "invoice_package.download",
    ),
    (
        "invoicePackage_summary",
        "GET",
        "/companies/{companyId}/invoice-package/summary",
        "invoice_package.summary",
    ),
    (
        "invoices_calculate",
        "POST",
        "/invoices/calculate",
        "invoices.calculate",
    ),
    (
        "invoices_cancel",
        "POST",
        "/invoices/{id}/cancel",
        "invoices.cancel",
    ),
    ("invoices_create", "POST", "/invoices", "invoices.create"),
    (
        "invoices_createBatch",
        "POST",
        "/invoices/batches",
        "invoices.create_batch",
    ),
    (
        "invoices_createDownloadUrl",
        "POST",
        "/invoices/{id}/download-url",
        "invoices.download_url",
    ),
    (
        "invoices_downloadPdf",
        "GET",
        "/invoices/{id}/pdf",
        "invoices.pdf",
    ),
    (
        "invoices_downloadXml",
        "GET",
        "/invoices/{id}/xml",
        "invoices.xml",
    ),
    ("invoices_get", "GET", "/invoices/{id}", "invoices.get"),
    ("invoices_list", "GET", "/invoices", "invoices.list"),
    (
        "invoices_listEvents",
        "GET",
        "/invoices/{id}/events",
        "invoices.events",
    ),
    (
        "invoices_overview",
        "GET",
        "/invoices/overview",
        "invoices.overview",
    ),
    (
        "invoices_review",
        "POST",
        "/invoices/review",
        "invoices.review",
    ),
    (
        "invoices_sendEmail",
        "POST",
        "/invoices/{id}/email",
        "invoices.send_email",
    ),
    (
        "invoices_substitute",
        "POST",
        "/invoices/{id}/substitute",
        "invoices.substitute",
    ),
    (
        "schedules_cancel",
        "POST",
        "/schedules/{id}/cancel",
        "schedules.cancel",
    ),
    ("schedules_get", "GET", "/schedules/{id}", "schedules.get"),
    ("schedules_list", "GET", "/schedules", "schedules.list"),
    (
        "schedules_listRuns",
        "GET",
        "/schedules/{id}/runs",
        "schedules.list_runs",
    ),
    (
        "schedules_pause",
        "POST",
        "/schedules/{id}/pause",
        "schedules.pause",
    ),
    (
        "schedules_resume",
        "POST",
        "/schedules/{id}/resume",
        "schedules.resume",
    ),
    (
        "schedules_update",
        "PATCH",
        "/schedules/{id}",
        "schedules.update",
    ),
    (
        "serviceFavorites_create",
        "POST",
        "/companies/{companyId}/service-favorites",
        "service_favorites.create",
    ),
    (
        "serviceFavorites_delete",
        "DELETE",
        "/companies/{companyId}/service-favorites/{favoriteId}",
        "service_favorites.remove",
    ),
    (
        "serviceFavorites_list",
        "GET",
        "/companies/{companyId}/service-favorites",
        "service_favorites.list",
    ),
    (
        "serviceTaxProfiles_approve",
        "POST",
        "/companies/{id}/service-tax-profiles/{serviceId}/approve",
        "service_tax_profiles.approve",
    ),
    (
        "serviceTaxProfiles_create",
        "POST",
        "/companies/{id}/service-tax-profiles",
        "service_tax_profiles.create",
    ),
    (
        "serviceTaxProfiles_delete",
        "DELETE",
        "/companies/{id}/service-tax-profiles/{serviceId}",
        "service_tax_profiles.remove",
    ),
    (
        "serviceTaxProfiles_list",
        "GET",
        "/companies/{id}/service-tax-profiles",
        "service_tax_profiles.list",
    ),
    (
        "serviceTaxProfiles_update",
        "PATCH",
        "/companies/{id}/service-tax-profiles/{serviceId}",
        "service_tax_profiles.update",
    ),
    (
        "webhookDeliveries_list",
        "GET",
        "/webhooks/deliveries",
        "webhooks.deliveries.list",
    ),
    (
        "webhookDeliveries_retry",
        "POST",
        "/webhooks/deliveries/{id}/retry",
        "webhooks.deliveries.retry",
    ),
    (
        "webhookDeliveries_stats",
        "GET",
        "/webhooks/deliveries/stats",
        "webhooks.deliveries.stats",
    ),
    ("webhooks_create", "POST", "/webhooks", "webhooks.create"),
    (
        "webhooks_delete",
        "DELETE",
        "/webhooks/{id}",
        "webhooks.remove",
    ),
    ("webhooks_list", "GET", "/webhooks", "webhooks.list"),
    (
        "webhooks_test",
        "POST",
        "/webhooks/{id}/test",
        "webhooks.test",
    ),
];

fn spec() -> Value {
    let raw = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/spec/openapi.json"))
        .expect("spec/openapi.json");
    serde_json::from_str(&raw).expect("valid JSON")
}

fn spec_operations() -> Vec<(String, String, String)> {
    let spec = spec();
    let mut out = Vec::new();
    for (p, item) in spec["paths"].as_object().unwrap() {
        for (m, op) in item.as_object().unwrap() {
            if let Some(id) = op.get("operationId").and_then(Value::as_str) {
                out.push((id.to_owned(), m.to_uppercase(), p.clone()));
            }
        }
    }
    out
}

#[test]
fn spec_is_contract_0_3_0_with_92_operations() {
    assert_eq!(spec()["info"]["version"], "0.3.0");
    assert_eq!(spec_operations().len(), 92);
}

#[test]
fn every_operation_of_the_contract_is_implemented() {
    let implemented: BTreeSet<&str> = IMPLEMENTED.iter().map(|r| r.0).collect();
    let missing: Vec<String> = spec_operations()
        .into_iter()
        .filter(|(id, _, _)| !implemented.contains(id.as_str()))
        .map(|(id, m, p)| format!("{id} ({m} {p})"))
        .collect();
    assert!(
        missing.is_empty(),
        "operations without an SDK method: {missing:#?}"
    );
}

#[test]
fn the_table_has_no_unknown_operation_and_matches_method_and_path() {
    let ops = spec_operations();
    for (id, verb, p, _) in IMPLEMENTED {
        let found = ops
            .iter()
            .find(|(oid, _, _)| oid == id)
            .unwrap_or_else(|| panic!("{id} is not in the contract"));
        assert_eq!((found.1.as_str(), found.2.as_str()), (*verb, *p), "{id}");
    }
}

#[test]
fn every_sdk_method_in_the_table_exists() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/src/resources");
    let mut source = String::new();
    for entry in fs::read_dir(dir).unwrap() {
        source.push_str(&fs::read_to_string(entry.unwrap().path()).unwrap());
    }
    for (id, _, _, sdk) in IMPLEMENTED {
        let name = sdk.rsplit('.').next().unwrap();
        assert!(
            source.contains(&format!("pub async fn {name}(")),
            "{id}: `pub async fn {name}` not found in src/resources"
        );
    }
}
