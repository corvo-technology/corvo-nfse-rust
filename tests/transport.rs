//! Transport-level tests: envelope unwrap, headers, pagination, error mapping.
//!
//! All against a local wiremock server — no network, no credentials.

use corvo_nfse::{
    Company, CompanyCreateParams, Corvo, CorvoError, Invoice, InvoiceCreateParams,
    InvoiceCustomerInput, InvoiceListParams, InvoiceServiceInput, InvoiceStatus, TaxRegime,
};
use serde_json::json;
use wiremock::matchers::{body_partial_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn client_for(server: &MockServer) -> Corvo {
    Corvo::builder()
        .api_key("corvo_test_0123456789abcdef0123456789abcdef01234567")
        .base_url(server.uri())
        .build()
        .expect("test client should build against a local server")
}

fn company_json(id: &str) -> serde_json::Value {
    json!({
        "id": id,
        "tenantId": "ten_1",
        "legalName": "Acme Ltd",
        "tradeName": "Acme",
        "taxId": "52998224725",
        "municipalReg": null,
        "municipalityCode": "3550308",
        "taxRegime": "Simples Nacional",
        "simpleNational": true,
        "specialRegime": null,
        "addressStreet": "Rua A",
        "addressNumber": "1",
        "addressComplement": null,
        "addressDistrict": "Centro",
        "addressCity": "Sao Paulo",
        "addressState": "SP",
        "addressZip": "01000000",
        "fiscalSchemaVersion": "1.0.0",
        "environment": "test",
        "status": "active",
        "createdAt": "2026-10-13T00:00:00Z",
        "updatedAt": "2026-10-13T00:00:00Z",
        "deletedAt": null,
        "setupGaps": []
    })
}

#[tokio::test]
async fn unwraps_data_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/companies/c_1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "data": company_json("c_1") })),
        )
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let company = corvo.companies().get("c_1").await.expect("get company");
    assert_eq!(company.id, "c_1");
    assert_eq!(company.legal_name, "Acme Ltd");
    assert_eq!(company.tax_regime, TaxRegime::SimplesNacional);
}

#[tokio::test]
async fn unwraps_data_props_envelope_for_invoices() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/invoices/inv_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": { "props": {
                "id": "inv_1",
                "companyId": "c_1",
                "tenantId": "ten_1",
                "environment": "test",
                "documentKeyVersion": 1,
                "emailSnapshot": { "send": true },
                "externalId": null,
                "idempotencyKey": null,
                "status": "authorized",
                "customerSnapshot": { "name": "Customer", "taxId": "52998224725" },
                "serviceSnapshot": { "code": "0107" },
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
                "emailStatus": "pending",
                "stage": "authorized"
            }}
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let invoice: Invoice = corvo.invoices().get("inv_1").await.expect("get invoice");
    assert_eq!(invoice.id, "inv_1");
    assert_eq!(invoice.status, InvoiceStatus::Authorized);
}

#[tokio::test]
async fn sends_api_key_header_and_v1_prefix() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/companies/c_1"))
        .and(header(
            "x-api-key",
            "corvo_test_0123456789abcdef0123456789abcdef01234567",
        ))
        .and(header("accept", "application/json"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "data": company_json("c_1") })),
        )
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    corvo.companies().get("c_1").await.expect("authorized get");
}

#[tokio::test]
async fn uses_default_base_url_and_custom_prefix() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/custom/companies/c_1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "data": company_json("c_1") })),
        )
        .mount(&server)
        .await;

    let corvo = Corvo::builder()
        .api_key("corvo_test_x")
        .base_url(server.uri())
        .prefix("custom")
        .build()
        .expect("builder");
    let company = corvo.companies().get("c_1").await.expect("get");
    assert_eq!(company.id, "c_1");
}

#[tokio::test]
async fn list_returns_data_and_pagination() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/companies"))
        .and(query_param("limit", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [company_json("c_1"), company_json("c_2")],
            "pagination": { "limit": 2, "hasMore": true, "nextCursor": "cur_2" }
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let page = corvo
        .companies()
        .list(corvo_nfse::CompanyListParams {
            limit: Some(2),
            cursor: None,
        })
        .await
        .expect("list companies");
    assert_eq!(page.data.len(), 2);
    assert!(page.pagination.has_more);
    assert_eq!(page.pagination.next_cursor.as_deref(), Some("cur_2"));
}

#[tokio::test]
async fn delete_204_maps_to_ok() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/v1/companies/c_1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    corvo.companies().remove("c_1").await.expect("delete");
}

#[tokio::test]
async fn rest_error_maps_to_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/companies"))
        .respond_with(ResponseTemplate::new(422).set_body_json(json!({
            "code": "ERROR_VALIDATION",
            "message": "Validation failed",
            "details": [{ "field": "taxId", "code": "invalid", "message": "bad tax id" }],
            "requestId": "req_42"
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let params = CompanyCreateParams {
        legal_name: "Acme".into(),
        tax_id: "1".into(),
        tax_regime: TaxRegime::Mei,
        trade_name: None,
        municipal_reg: None,
        municipality_code: None,
        simple_national: None,
        special_regime: None,
        address: None,
    };
    let err = corvo
        .companies()
        .create(&params)
        .await
        .expect_err("validation error");
    match &err {
        CorvoError::Api {
            status,
            code,
            message,
            details,
            request_id,
            retry_after,
        } => {
            assert_eq!(*status, 422);
            assert_eq!(code, "ERROR_VALIDATION");
            assert_eq!(message, "Validation failed");
            assert_eq!(details.len(), 1);
            assert_eq!(details[0].field, "taxId");
            assert_eq!(request_id.as_deref(), Some("req_42"));
            assert_eq!(*retry_after, None);
        }
        other => panic!("expected Api error, got {other:?}"),
    }
    assert_eq!(err.status(), 422);
    assert_eq!(err.code(), "ERROR_VALIDATION");
    assert_eq!(err.request_id(), Some("req_42"));
}

#[tokio::test]
async fn rate_limit_error_exposes_retry_after() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/credits"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "7")
                .set_body_json(json!({ "code": "RATE_LIMITED", "message": "slow down" })),
        )
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let err = corvo.credits().balance().await.expect_err("429");
    match &err {
        CorvoError::Api { retry_after, .. } => assert_eq!(*retry_after, Some(7)),
        other => panic!("expected Api error, got {other:?}"),
    }
}

#[tokio::test]
async fn non_json_error_body_falls_back_to_status() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/credits"))
        .respond_with(
            ResponseTemplate::new(502).set_body_raw("<html>bad gateway</html>", "text/html"),
        )
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let err = corvo.credits().balance().await.expect_err("502");
    match &err {
        CorvoError::Api {
            status,
            code,
            message,
            ..
        } => {
            assert_eq!(*status, 502);
            assert_eq!(code, "http_502");
            assert!(message.contains("502"));
        }
        other => panic!("expected Api error, got {other:?}"),
    }
}

#[tokio::test]
async fn invalid_json_success_body_is_error_not_panic() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/credits"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not json", "application/json"))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let err = corvo.credits().balance().await.expect_err("invalid json");
    match &err {
        CorvoError::Api { code, .. } => assert_eq!(code, "invalid_response"),
        other => panic!("expected Api error, got {other:?}"),
    }
}

#[tokio::test]
async fn iterate_walks_all_pages() {
    let server = MockServer::start().await;
    // Mount the cursor-specific mock first; wiremock matches in mount order.
    Mock::given(method("GET"))
        .and(path("/v1/companies"))
        .and(query_param("cursor", "cur_1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [company_json("c_2")],
            "pagination": { "limit": 1, "hasMore": false, "nextCursor": null }
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/companies"))
        .and(query_param("limit", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [company_json("c_1")],
            "pagination": { "limit": 1, "hasMore": true, "nextCursor": "cur_1" }
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let items = corvo
        .companies()
        .iterate(corvo_nfse::CompanyListParams {
            limit: Some(1),
            cursor: None,
        })
        .collect()
        .await
        .expect("iterate");
    let ids: Vec<&str> = items.iter().map(|c| c.id.as_str()).collect();
    assert_eq!(ids, vec!["c_1", "c_2"]);
}

#[tokio::test]
async fn iterate_surfaces_first_page_failure_as_err() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/companies"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
            "code": "INTERNAL", "message": "boom"
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let result: Result<Vec<Company>, _> = corvo
        .companies()
        .iterate(Default::default())
        .collect()
        .await;
    match result {
        Err(CorvoError::Api { status, code, .. }) => {
            assert_eq!(status, 500);
            assert_eq!(code, "INTERNAL");
        }
        other => panic!("expected Api error, got {other:?}"),
    }
}

#[tokio::test]
async fn invoice_create_sends_camel_case_body_and_idempotency_key() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/invoices"))
        .and(header("idempotency-key", "idem_1"))
        .and(body_partial_json(json!({
            "companyId": "c_1",
            "competenceDate": "2026-10-13",
            "amount": 10.5,
            "customer": { "name": "Customer", "taxId": "52998224725" },
            "service": { "code": "0107" }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": { "props": {
                "id": "inv_1", "companyId": "c_1", "tenantId": "ten_1",
                "environment": "test", "documentKeyVersion": 1,
                "emailSnapshot": { "send": false }, "externalId": null,
                "idempotencyKey": "idem_1", "status": "queued",
                "customerSnapshot": { "name": "Customer" }, "serviceSnapshot": { "code": "0107" },
                "taxSnapshot": {}, "amount": 10.5, "currency": "BRL",
                "competenceDate": "2026-10-13",
                "dpsId": null, "dpsSeries": null, "dpsNumber": null,
                "accessKey": null, "nfseNumber": null,
                "fiscalProvider": null, "fiscalSchemaVersion": null,
                "issuedAt": null, "authorizedAt": null,
                "lastErrorCode": null, "lastErrorMessage": null, "lastErrorProvider": null,
                "documentsReadyAt": null, "lastEmailStatus": null, "lastEmailSentAt": null,
                "createdAt": "2026-10-13T00:00:00Z", "updatedAt": "2026-10-13T00:00:00Z",
                "documentsStatus": "pending", "emailStatus": "pending", "stage": "pending"
            }}
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let params = InvoiceCreateParams {
        company_id: Some("c_1".into()),
        company_tax_id: None,
        external_id: None,
        correlation_id: None,
        customer: InvoiceCustomerInput {
            name: "Customer".into(),
            tax_id: "52998224725".into(),
            email: None,
            address: None,
        },
        service: InvoiceServiceInput {
            code: "0107".into(),
            municipality_code: None,
            description: None,
        },
        amount: 10.5,
        competence_date: "2026-10-13".into(),
        email: None,
        options: None,
    };
    let invoice = corvo
        .invoices()
        .create(&params, Some("idem_1"))
        .await
        .expect("create invoice");
    assert_eq!(invoice.id, "inv_1");
    assert_eq!(invoice.status, InvoiceStatus::Queued);
}

#[tokio::test]
async fn invoice_list_query_params_pass_through() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/invoices"))
        .and(query_param("companyId", "c_1"))
        .and(query_param("status", "authorized"))
        .and(query_param("limit", "3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [],
            "pagination": { "limit": 3, "hasMore": false, "nextCursor": null }
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let page = corvo
        .invoices()
        .list(&InvoiceListParams {
            company_id: Some("c_1".into()),
            status: Some(InvoiceStatus::Authorized),
            from: None,
            to: None,
            external_id: None,
            limit: Some(3),
            cursor: None,
        })
        .await
        .expect("list invoices");
    assert!(page.data.is_empty());
}

#[tokio::test]
async fn http_public_base_url_is_rejected_by_default() {
    let err = Corvo::new("corvo_test_x", "http://api.example.com").expect_err("http rejected");
    assert!(matches!(err, CorvoError::InvalidUrl(_)));
}

#[tokio::test]
async fn http_loopback_base_url_is_allowed() {
    let server = MockServer::start().await;
    let corvo = Corvo::new("corvo_test_x", server.uri()).expect("loopback http allowed");
    let _ = corvo.health();
}

#[tokio::test]
async fn missing_api_key_is_config_error() {
    // Only meaningful when CORVO_API_KEY is not set in the environment.
    if std::env::var("CORVO_API_KEY").is_ok() {
        return;
    }
    let err = Corvo::builder().build().expect_err("missing key");
    assert!(matches!(err, CorvoError::Config(_)));
}

#[tokio::test]
async fn timeout_maps_to_timeout_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/credits"))
        .respond_with(ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(5)))
        .mount(&server)
        .await;

    let corvo = Corvo::builder()
        .api_key("corvo_test_x")
        .base_url(server.uri())
        .timeout(std::time::Duration::from_millis(100))
        .build()
        .expect("builder");
    let err = corvo.credits().balance().await.expect_err("timeout");
    assert!(matches!(err, CorvoError::Timeout), "got {err:?}");
}
