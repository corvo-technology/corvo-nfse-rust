//! Batch issuance (`invoices().create_batch()`), `deduction` and the Simples
//! rate fields.

use corvo_nfse::{
    CompanyCreateParams, Corvo, InvoiceBatchItem, InvoiceCreateParams, InvoiceServiceInput,
    TaxRegime,
};
use pretty_assertions::assert_eq;
use serde_json::json;
use wiremock::matchers::{body_json, method, path};
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

fn item(key: &str) -> InvoiceBatchItem {
    InvoiceBatchItem {
        invoice: InvoiceCreateParams {
            company_tax_id: Some("11222333000181".into()),
            service: InvoiceServiceInput {
                description: Some("Consultoria".into()),
                ..Default::default()
            },
            amount: 150.5,
            deduction: Some(20.0),
            competence_date: "2026-10-01".into(),
            ..Default::default()
        },
        idempotency_key: Some(key.into()),
    }
}

#[tokio::test]
async fn create_batch_posts_the_items_and_reads_accepted_replayed_and_rejected() {
    let server = MockServer::start().await;
    let mut saved = invoice_json("inv_1", "queued");
    saved["deduction"] = json!(20.0);
    Mock::given(method("POST"))
        .and(path("/v1/invoices/batches"))
        .and(body_json(json!({
            "invoices": [{
                "companyTaxId": "11222333000181",
                "service": { "description": "Consultoria" },
                "amount": 150.5,
                "competenceDate": "2026-10-01",
                "deduction": 20.0,
                "idempotencyKey": "sheet-row-2"
            }]
        })))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "data": {
                "total": 2, "accepted": 1, "rejected": 1,
                "results": [
                    { "index": 0, "status": "accepted", "replayed": true, "invoice": saved },
                    { "index": 1, "status": "rejected",
                      "error": { "code": "ERROR_VALIDATION", "message": "Request validation failed",
                                 "details": [{ "field": "amount", "code": "ERROR_OUT_OF_RANGE", "message": "x" }] } }
                ]
            }
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let batch = corvo
        .invoices()
        .create_batch(&[item("sheet-row-2")])
        .await
        .expect("batch");
    assert_eq!(batch.total, 2);
    assert_eq!(batch.results[0].replayed, Some(true));
    let invoice = batch.results[0].invoice.as_ref().expect("invoice");
    assert_eq!(invoice.id, "inv_1");
    assert_eq!(invoice.deduction, Some(20.0));
    assert_eq!(
        batch.results[1].error.as_ref().expect("error").code,
        "ERROR_VALIDATION"
    );

    let requests = server.received_requests().await.expect("requests");
    assert!(requests[0].headers.get("idempotency-key").is_none());
}

#[tokio::test]
async fn company_create_sends_the_simples_rates() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/companies"))
        .and(body_json(json!({
            "legalName": "Acme",
            "taxId": "11222333000181",
            "taxRegime": "Simples Nacional",
            "municipalityCode": "3550308",
            "simplesRate": "6.54",
            "simplesIssRate": "3.00"
        })))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "statusCode": 400, "code": "ERROR_VALIDATION", "message": "stop here"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let _ = corvo
        .companies()
        .create(&CompanyCreateParams {
            legal_name: "Acme".into(),
            tax_id: "11222333000181".into(),
            tax_regime: TaxRegime::SimplesNacional,
            municipality_code: "3550308".into(),
            trade_name: None,
            municipal_reg: None,
            send_municipal_reg: None,
            default_service_code: None,
            simple_national: None,
            special_regime: None,
            simples_rate: Some("6.54".into()),
            simples_iss_rate: Some("3.00".into()),
            address: None,
        })
        .await;
}
