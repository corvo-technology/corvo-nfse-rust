//! Unit tests of the operations added or changed by API contract 0.3.0, with
//! mocked HTTP: method, path, query, headers, body and parsing.

use std::collections::BTreeMap;

use corvo_nfse::*;
use serde_json::{json, Value};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const KEY: &str = "corvo_test_0123456789abcdef0123456789abcdef01234567";
const UUID: &str = "11111111-1111-4111-8111-111111111111";
const UUID2: &str = "22222222-2222-4222-8222-222222222222";

/// A mock server answering `status` with `body` on `verb path`, and a client for it.
async fn setup(verb: &str, p: &str, status: u16, body: Value) -> (MockServer, Corvo) {
    let server = MockServer::start().await;
    let template = if body.is_null() {
        ResponseTemplate::new(status)
    } else {
        ResponseTemplate::new(status).set_body_json(body)
    };
    Mock::given(method(verb))
        .and(path(p))
        .respond_with(template)
        .mount(&server)
        .await;
    let corvo = Corvo::builder()
        .api_key(KEY)
        .base_url(server.uri())
        .build()
        .unwrap();
    (server, corvo)
}

/// The only request the server received, with its JSON body (or null).
async fn sent(server: &MockServer) -> (wiremock::Request, Value) {
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1, "exactly one request");
    let request = requests.into_iter().next().unwrap();
    let body = serde_json::from_slice(&request.body).unwrap_or(Value::Null);
    (request, body)
}

fn header(request: &wiremock::Request, name: &str) -> Option<String> {
    request
        .headers
        .get(name)
        .map(|v| v.to_str().unwrap().to_owned())
}

fn query(request: &wiremock::Request) -> BTreeMap<String, String> {
    request.url.query_pairs().into_owned().collect()
}

fn page(items: Value, next: Option<&str>) -> Value {
    json!({
        "data": items,
        "pagination": { "limit": 50, "hasMore": next.is_some(), "nextCursor": next }
    })
}

fn api_error(status: u16, code: &str) -> Value {
    let _ = status;
    json!({ "statusCode": status, "code": code, "message": "nope", "requestId": UUID })
}

fn invoice(id: &str) -> Value {
    json!({
        "id": id, "companyId": UUID, "tenantId": UUID2, "environment": "test",
        "externalId": null, "idempotencyKey": null, "status": "queued",
        "documentsStatus": "pending", "emailStatus": null, "stage": "pending",
        "customerSnapshot": { "name": "Cliente" },
        "serviceSnapshot": { "code": "010101", "description": "S", "municipalityCode": "3550308",
                              "purchaseOrder": "PO-1", "notes": "n" },
        "taxSnapshot": {}, "amount": 10.5, "currency": "BRL",
        "competenceDate": "2026-11-24T00:00:00.000Z",
        "dpsId": null, "dpsSeries": null, "dpsNumber": null, "accessKey": null,
        "nfseNumber": null, "fiscalProvider": "nacional", "fiscalSchemaVersion": "1.01",
        "issuedAt": null, "authorizedAt": null, "lastErrorCode": null,
        "lastErrorMessage": null, "lastErrorProvider": null, "documentsReadyAt": null,
        "lastEmailStatus": null, "lastEmailSentAt": null,
        "createdAt": "2026-11-24T00:00:00Z", "updatedAt": "2026-11-24T00:00:00Z",
        "substitutesInvoiceId": UUID, "substitutionDescription": "motivo", "portalFlag": "taker_rejected"
    })
}

fn schedule() -> Value {
    json!({
        "id": UUID, "company_id": UUID2, "kind": "recurring", "status": "active",
        "anchor_on": "2026-11-25", "ends_on": null, "frequency": "MONTHLY",
        "next_run_on": "2026-11-25", "last_run_on": null, "attempts": 0,
        "pause_reason": null, "invoice": { "amount": 10 }, "created_at": "2026-11-24T00:00:00Z"
    })
}

fn saved_customer() -> Value {
    json!({
        "taxId": "52998224725", "name": "Maria", "email": null,
        "lastUsedAt": "2026-11-24T00:00:00Z", "createdAt": "2026-11-24T00:00:00Z",
        "updatedAt": "2026-11-24T00:00:00Z"
    })
}

fn favorite() -> Value {
    json!({
        "id": UUID, "tenantId": UUID2, "companyId": UUID2, "label": "Consultoria",
        "serviceCode": "010101", "description": "Consultoria", "amountCents": 15000,
        "municipalityCode": null, "createdAt": "2026-11-24T00:00:00Z"
    })
}

fn import_job() -> Value {
    json!({
        "id": UUID, "company_id": null, "file_name": "a.csv", "status": "ready",
        "rows_expected": 2, "unknown_columns": ["x"], "error_code": null,
        "created_at": "2026-11-24T00:00:00Z", "emit_started_at": null, "emit_finished_at": null,
        "counters": { "total": 2, "valid": 2, "invalid": 0, "accepted": 0, "pending": 0,
            "authorized": 0, "failed": 0, "awaiting_credits": 0, "authorized_amount": 0,
            "awaiting_amount": 0, "failed_amount": 0, "pending_amount": 0, "valid_amount": 20.5 }
    })
}

fn connection() -> Value {
    json!({
        "id": UUID, "provider": "stripe", "environment": "test", "companyId": UUID2,
        "state": "draft", "externalAccountId": null, "activeFrom": null,
        "webhookUrl": "https://api.corvo.tech/v1/integrations/inbound/x",
        "createdAt": "2026-11-24T00:00:00Z"
    })
}

fn operation() -> Value {
    json!({
        "id": UUID, "amountCents": 1000, "blockReason": null, "commercialState": "paid",
        "currency": "BRL", "customer": { "name": null, "email": null, "taxId": null, "taxIdMasked": false },
        "fiscalStatus": "blocked", "invoiceId": null, "operationKey": "k", "paidAt": null,
        "createdAt": "2026-11-24T00:00:00Z", "updatedAt": "2026-11-24T00:00:00Z"
    })
}

fn tax_profile() -> Value {
    json!({
        "id": UUID, "tenantId": UUID2, "companyId": UUID2, "name": "Perfil", "serviceCode": "010101",
        "catalogVersion": null, "classCode": null, "classificationSource": "manual", "cst": null,
        "effectiveFrom": "2026-01-01", "effectiveUntil": null, "incidenceMunicipalityCode": null,
        "issRate": "2.00", "issTaxation": "NORMAL", "issWithholdingDefault": false,
        "municipalServiceCode": null, "nbsCode": null, "operationIndicator": null,
        "operationNature": "SERVICE", "regimeApplicability": ["MEI"], "reviewStatus": "unreviewed",
        "withholdingPolicy": "NONE_CONFIRMED", "withholdingRates": {}, "createdAt": "2026-11-24T00:00:00Z"
    })
}

fn fiscal_profile() -> Value {
    json!({
        "id": UUID, "tenantId": UUID2, "companyId": UUID2, "taxRegime": "SIMPLES_NACIONAL",
        "simpleNational": true, "simplesApuracao": "SN", "simplesIssRate": null,
        "specialTaxRegime": null, "ibsCbsRegularOption": false, "ibsCbsRegularFrom": null,
        "source": "api", "reviewStatus": "approved", "reviewedBy": null, "reviewedAt": null,
        "validFrom": "2026-01-01", "validUntil": null, "createdAt": "2026-11-24T00:00:00Z"
    })
}

// ---------------------------------------------------------------- customers

#[tokio::test]
async fn customers_search_sends_query_and_paginates() {
    let (server, corvo) = setup(
        "GET",
        "/v1/customers",
        200,
        page(json!([saved_customer()]), Some("50")),
    )
    .await;
    let result = corvo
        .customers()
        .search(&CustomerSearchParams {
            search: Some("mar".into()),
            limit: Some(5),
            cursor: Some("10".into()),
        })
        .await
        .unwrap();
    assert_eq!(result.data[0].tax_id, "52998224725");
    assert!(result.pagination.has_more);
    assert_eq!(result.pagination.next_cursor.as_deref(), Some("50"));
    let (req, _) = sent(&server).await;
    assert_eq!(header(&req, "x-api-key").as_deref(), Some(KEY));
    let q = query(&req);
    assert_eq!(q["search"], "mar");
    assert_eq!(q["limit"], "5");
    assert_eq!(q["cursor"], "10");
}

#[tokio::test]
async fn customers_iterate_follows_the_cursor() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/customers"))
        .and(wiremock::matchers::query_param("cursor", "50"))
        .respond_with(ResponseTemplate::new(200).set_body_json(page(json!([]), None)))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/customers"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(page(json!([saved_customer()]), Some("50"))),
        )
        .mount(&server)
        .await;
    let corvo = Corvo::new(KEY, server.uri()).unwrap();
    let all = corvo
        .customers()
        .iterate(&CustomerSearchParams::default())
        .collect()
        .await
        .unwrap();
    assert_eq!(all.len(), 1);
}

#[tokio::test]
async fn customers_get_update_remove() {
    let (server, corvo) = setup(
        "GET",
        "/v1/customers/52998224725",
        200,
        json!({ "data": saved_customer() }),
    )
    .await;
    assert_eq!(
        corvo.customers().get("52998224725").await.unwrap().name,
        "Maria"
    );
    drop(server);

    let (server, corvo) = setup(
        "PATCH",
        "/v1/customers/52998224725",
        200,
        json!({ "data": saved_customer() }),
    )
    .await;
    corvo
        .customers()
        .update(
            "52998224725",
            &CustomerUpdateParams {
                name: Some("Maria S".into()),
                email: Some(None),
            },
        )
        .await
        .unwrap();
    let (_, body) = sent(&server).await;
    assert_eq!(body, json!({ "name": "Maria S", "email": null }));

    let (server, corvo) = setup(
        "DELETE",
        "/v1/customers/52998224725",
        200,
        json!({ "data": null }),
    )
    .await;
    corvo.customers().remove("52998224725").await.unwrap();
    assert_eq!(sent(&server).await.0.method.as_str(), "DELETE");
}

#[tokio::test]
async fn customers_get_not_found_is_an_error_value() {
    let (_s, corvo) = setup(
        "GET",
        "/v1/customers/52998224725",
        404,
        api_error(404, "ERROR_CUSTOMER_NOT_FOUND"),
    )
    .await;
    let err = corvo.customers().get("52998224725").await.unwrap_err();
    assert_eq!(err.status(), 404);
    assert_eq!(err.code(), "ERROR_CUSTOMER_NOT_FOUND");
    assert_eq!(err.request_id(), Some(UUID));
}

#[tokio::test]
async fn cnpj_lookup_parses_the_profile_and_maps_errors() {
    let (_s, corvo) = setup(
        "GET",
        "/v1/cnpj/11222333000181",
        200,
        json!({ "data": {
            "active": true, "legalName": "ACME LTDA", "tradeName": "Acme", "situation": "ATIVA",
            "mei": false, "simples": true, "municipalityCode": "3550308",
            "address": { "city": "Sao Paulo", "district": "Centro", "number": "1",
                          "state": "SP", "street": "Rua A", "zip": "01000000" }
        } }),
    )
    .await;
    let profile = corvo.cnpj().lookup("11222333000181").await.unwrap();
    assert_eq!(profile.legal_name, "ACME LTDA");
    assert_eq!(profile.simples, Some(true));
    assert_eq!(profile.address.state, "SP");

    let (_s, corvo) = setup("GET", "/v1/cnpj/123", 422, api_error(422, "CNPJ_INVALID")).await;
    assert_eq!(
        corvo.cnpj().lookup("123").await.unwrap_err().code(),
        "CNPJ_INVALID"
    );
}

// --------------------------------------------------------- service favorites

#[tokio::test]
async fn service_favorites_list_create_remove() {
    let p = format!("/v1/companies/{UUID}/service-favorites");
    let (_s, corvo) = setup("GET", &p, 200, json!({ "data": [favorite()] })).await;
    let list = corvo.service_favorites().list(UUID).await.unwrap();
    assert_eq!(list[0].amount_cents, Some(15000));

    let (server, corvo) = setup("POST", &p, 201, json!({ "data": favorite() })).await;
    corvo
        .service_favorites()
        .create(
            UUID,
            &ServiceFavoriteCreateParams {
                label: "Consultoria".into(),
                service_code: "010101".into(),
                description: "Consultoria".into(),
                amount_cents: Some(15000),
                municipality_code: None,
            },
        )
        .await
        .unwrap();
    let (_, body) = sent(&server).await;
    assert_eq!(
        body,
        json!({ "label": "Consultoria", "serviceCode": "010101",
                "description": "Consultoria", "amountCents": 15000 })
    );

    let (_s, corvo) = setup(
        "DELETE",
        &format!("{p}/{UUID2}"),
        200,
        json!({ "data": { "deleted": true } }),
    )
    .await;
    assert!(
        corvo
            .service_favorites()
            .remove(UUID, UUID2)
            .await
            .unwrap()
            .deleted
    );
}

// ---------------------------------------------------------- accountant package

#[tokio::test]
async fn accountant_package_get_update_send_list() {
    let p = format!("/v1/companies/{UUID}/accountant-package");
    let (_s, corvo) = setup(
        "GET",
        &p,
        200,
        json!({ "data": { "email": "c@x.com", "lastSentMonth": "2026-10" } }),
    )
    .await;
    let s = corvo.accountant_package().get(UUID).await.unwrap();
    assert_eq!(s.email.as_deref(), Some("c@x.com"));

    let (server, corvo) = setup(
        "PUT",
        &p,
        200,
        json!({ "data": { "email": null, "lastSentMonth": null, "testEmailSent": false } }),
    )
    .await;
    let u = corvo.accountant_package().update(UUID, None).await.unwrap();
    assert!(!u.test_email_sent);
    let (_, body) = sent(&server).await;
    assert_eq!(body, json!({ "accountantEmail": null }));

    let (server, corvo) = setup(
        "POST",
        &format!("{p}/send"),
        200,
        json!({ "data": { "status": "skipped-already-sent" } }),
    )
    .await;
    let r = corvo
        .accountant_package()
        .send(UUID, "2026-10")
        .await
        .unwrap();
    assert_eq!(r.status, AccountantSendOutcome::SkippedAlreadySent);
    assert_eq!(sent(&server).await.1, json!({ "month": "2026-10" }));

    let (_s, corvo) = setup(
        "GET",
        &format!("{p}/sends"),
        200,
        json!({ "data": [{ "id": UUID, "createdAt": "2026-11-01T00:00:00Z", "invoiceCount": 3,
            "month": "2026-10", "recipient": "c@x.com", "status": "sent", "totalCents": 1000,
            "trigger": "manual" }] }),
    )
    .await;
    assert_eq!(
        corvo.accountant_package().list_sends(UUID).await.unwrap()[0].invoice_count,
        3
    );
}

#[tokio::test]
async fn accountant_package_too_large_is_an_error_code() {
    let (_s, corvo) = setup(
        "POST",
        &format!("/v1/companies/{UUID}/accountant-package/send"),
        422,
        api_error(422, "PACKAGE_TOO_LARGE"),
    )
    .await;
    let err = corvo
        .accountant_package()
        .send(UUID, "2026-10")
        .await
        .unwrap_err();
    assert_eq!(err.code(), "PACKAGE_TOO_LARGE");
}

// ------------------------------------------------------------ invoice package

#[tokio::test]
async fn invoice_package_summary_and_download() {
    let p = format!("/v1/companies/{UUID}/invoice-package");
    let (server, corvo) = setup(
        "GET",
        &format!("{p}/summary"),
        200,
        json!({ "data": { "companyName": "Acme", "companyTaxId": "11222333000181",
            "count": 4, "limit": 500, "month": "2026-10", "total": "1234.50" } }),
    )
    .await;
    let s = corvo
        .invoice_package()
        .summary(UUID, "2026-10")
        .await
        .unwrap();
    assert_eq!(s.count, 4);
    assert_eq!(s.total, "1234.50");
    assert_eq!(query(&sent(&server).await.0)["month"], "2026-10");

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(p))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/zip")
                .insert_header(
                    "content-disposition",
                    "attachment; filename=\"pacote-2026-10.zip\"",
                )
                .set_body_bytes(b"PK\x03\x04zip".to_vec()),
        )
        .mount(&server)
        .await;
    let corvo = Corvo::new(KEY, server.uri()).unwrap();
    let file = corvo
        .invoice_package()
        .download(UUID, "2026-10", Some(&[PackagePart::Xml, PackagePart::Pdf]))
        .await
        .unwrap();
    assert_eq!(file.bytes, b"PK\x03\x04zip");
    assert_eq!(file.mime_type.as_deref(), Some("application/zip"));
    assert_eq!(file.file_name.as_deref(), Some("pacote-2026-10.zip"));
    let (req, _) = sent(&server).await;
    assert_eq!(query(&req)["include"], "xml,pdf");
    assert_eq!(header(&req, "accept").as_deref(), Some("application/zip"));
}

// ------------------------------------------------------------------- invoices

#[tokio::test]
async fn invoices_substitute_sends_idempotency_key_and_body() {
    let (server, corvo) = setup(
        "POST",
        &format!("/v1/invoices/{UUID}/substitute"),
        202,
        json!({ "data": invoice(UUID2) }),
    )
    .await;
    let mut params = InvoiceSubstituteParams::new("Descricao corrigida do servico");
    params.purchase_order = Some("PO-2".into());
    params.service = Some(SubstituteService {
        description: Some("novo".into()),
    });
    let inv = corvo
        .invoices()
        .substitute(UUID, &params, Some("sub-1"))
        .await
        .unwrap();
    assert_eq!(inv.substitutes_invoice_id.as_deref(), Some(UUID));
    assert_eq!(inv.portal_flag, Some(PortalFlag::TakerRejected));
    assert_eq!(inv.service_snapshot.purchase_order.as_deref(), Some("PO-1"));
    let (req, body) = sent(&server).await;
    assert_eq!(header(&req, "idempotency-key").as_deref(), Some("sub-1"));
    assert_eq!(
        body,
        json!({ "description": "Descricao corrigida do servico", "purchaseOrder": "PO-2",
                "service": { "description": "novo" } })
    );
}

#[tokio::test]
async fn invoices_overview_and_review() {
    let (server, corvo) = setup(
        "GET",
        "/v1/invoices/overview",
        200,
        json!({ "data": { "counts": { "authorized": 3, "processing": 1, "rejected": 0 },
            "dailyActivity": [{ "date": "2026-11-24", "count": 4 }], "from": "a", "to": "b",
            "period": "7d", "timezone": "America/Sao_Paulo", "recentInvoices": [invoice(UUID)] } }),
    )
    .await;
    let o = corvo
        .invoices()
        .overview(Some(OverviewPeriod::SevenDays))
        .await
        .unwrap();
    assert_eq!(o.counts.authorized, 3);
    assert_eq!(o.recent_invoices.len(), 1);
    assert_eq!(query(&sent(&server).await.0)["period"], "7d");

    let (server, corvo) = setup(
        "POST",
        "/v1/invoices/review",
        202,
        json!({ "data": { "results": [
            { "id": UUID, "outcome": "accepted" },
            { "id": UUID2, "outcome": "skipped", "reason": "not_in_manual_review" } ] } }),
    )
    .await;
    let r = corvo
        .invoices()
        .review(&InvoiceReviewParams {
            action: ReviewAction::Retry,
            invoice_ids: vec![UUID.into(), UUID2.into()],
        })
        .await
        .unwrap();
    assert_eq!(r.results[1].reason.as_deref(), Some("not_in_manual_review"));
    assert_eq!(
        sent(&server).await.1,
        json!({ "action": "retry", "invoiceIds": [UUID, UUID2] })
    );
}

#[tokio::test]
async fn invoices_create_schedule_answers_a_schedule() {
    let (server, corvo) = setup("POST", "/v1/invoices", 202, json!({ "data": schedule() })).await;
    let params = InvoiceCreateParams {
        company_id: Some(UUID2.into()),
        amount: 10.0,
        competence_date: "2026-11-25".into(),
        purchase_order: Some("PO".into()),
        notes: Some("obs".into()),
        customer: Some(InvoiceCustomerInput {
            name: Some("Joe".into()),
            foreign: Some(ForeignCustomer {
                tax_id: None,
                no_tax_id_reason: Some(NoTaxIdReason::NotRequired),
                country: "US".into(),
                postal_code: "10001".into(),
                city: "NYC".into(),
                region: "NY".into(),
                street: "Broadway".into(),
                number: "1".into(),
                district: "Manhattan".into(),
                complement: None,
            }),
            ..Default::default()
        }),
        options: Some(InvoiceOptions {
            invoice_type: Some(InvoiceType::Recurring),
            recurrence: Some(RecurrenceOptions {
                frequency: Frequency::Monthly,
                ends_on: Some("2027-01-01".into()),
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    let created = corvo.invoices().create(&params, Some("k1")).await.unwrap();
    let sch = created.schedule().expect("a schedule");
    assert_eq!(sch.kind, ScheduleKind::Recurring);
    assert_eq!(sch.frequency, Some(Frequency::Monthly));
    assert!(created.invoice().is_none());
    let (req, body) = sent(&server).await;
    assert_eq!(header(&req, "idempotency-key").as_deref(), Some("k1"));
    assert_eq!(body["options"]["type"], "RECURRING");
    assert_eq!(
        body["options"]["recurrence"],
        json!({ "frequency": "MONTHLY", "endsOn": "2027-01-01" })
    );
    assert_eq!(body["purchaseOrder"], "PO");
    assert_eq!(body["notes"], "obs");
    assert_eq!(body["customer"]["foreign"]["noTaxIdReason"], "not_required");
    assert!(body["customer"].get("taxId").is_none());
}

#[tokio::test]
async fn invoices_create_immediate_answers_an_invoice() {
    let (_s, corvo) = setup(
        "POST",
        "/v1/invoices",
        202,
        json!({ "data": invoice(UUID) }),
    )
    .await;
    let created = corvo
        .invoices()
        .create(&InvoiceCreateParams::default(), None)
        .await
        .unwrap();
    assert_eq!(created.invoice().unwrap().id, UUID);
    assert!(created.schedule().is_none());
}

#[tokio::test]
async fn invoices_send_email_optional_destination_and_idempotency() {
    let (server, corvo) = setup(
        "POST",
        &format!("/v1/invoices/{UUID}/email"),
        202,
        json!({ "data": { "emailId": "e1", "status": "queued" } }),
    )
    .await;
    let r = corvo
        .invoices()
        .send_email(UUID, Some("outro@x.com"), Some("em-1"))
        .await
        .unwrap();
    assert_eq!(r.email_id, "e1");
    let (req, body) = sent(&server).await;
    assert_eq!(header(&req, "idempotency-key").as_deref(), Some("em-1"));
    assert_eq!(body, json!({ "email": "outro@x.com" }));
}

#[tokio::test]
async fn invoices_send_email_without_destination_sends_no_body() {
    let (server, corvo) = setup(
        "POST",
        &format!("/v1/invoices/{UUID}/email"),
        202,
        json!({ "data": { "emailId": "e1", "status": "queued" } }),
    )
    .await;
    corvo.invoices().send_email(UUID, None, None).await.unwrap();
    assert!(sent(&server).await.0.body.is_empty());
}

#[tokio::test]
async fn invoices_list_filters_by_customer_tax_id() {
    let (server, corvo) = setup("GET", "/v1/invoices", 200, page(json!([]), None)).await;
    corvo
        .invoices()
        .list(&InvoiceListParams {
            customer_tax_id: Some("52998224725".into()),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(
        query(&sent(&server).await.0)["customerTaxId"],
        "52998224725"
    );
}

#[tokio::test]
async fn invoices_calculate_unwraps_the_data_envelope() {
    let (_s, corvo) = setup(
        "POST",
        "/v1/invoices/calculate",
        200,
        json!({ "data": { "status": "ready", "preEmission": true,
            "taxes": { "iss": { "base": "10.00", "rate": "2", "amount": "0.20" } },
            "withholdings": [{ "tax": "ISS", "base": "1", "rate": "2", "amount": "0.02" }],
            "warnings": [{ "code": "W", "message": "m", "step": "s" }],
            "trace": [{ "step": "a", "status": "applied", "durationMs": 1.5 }] } }),
    )
    .await;
    let c = corvo
        .invoices()
        .calculate(&InvoiceCreateParams::default())
        .await
        .unwrap();
    assert_eq!(c.status, "ready");
    assert_eq!(c.withholdings[0].tax, "ISS");
    assert_eq!(c.trace[0].duration_ms, 1.5);
}

#[tokio::test]
async fn invoices_calculate_accepts_null_sections() {
    let (_s, corvo) = setup(
        "POST",
        "/v1/invoices/calculate",
        200,
        json!({ "data": { "status": "blocked", "taxRegime": null, "classification": null,
            "applicability": null, "calculation": null, "blocking": null } }),
    )
    .await;
    let c = corvo
        .invoices()
        .calculate(&InvoiceCreateParams::default())
        .await
        .unwrap();
    assert_eq!(c.status, "blocked");
    assert!(c.classification.is_null() && c.applicability.is_null() && c.calculation.is_null());
}

#[tokio::test]
async fn invoice_rate_limit_error_keeps_retry_after() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/invoices"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "7")
                .set_body_json(api_error(429, "ERROR_RATE_LIMITED")),
        )
        .mount(&server)
        .await;
    let corvo = Corvo::new(KEY, server.uri()).unwrap();
    let err = corvo
        .invoices()
        .create(&InvoiceCreateParams::default(), None)
        .await
        .unwrap_err();
    assert_eq!(err.status(), 429);
    assert_eq!(err.retry_after(), Some(7));
}

// ------------------------------------------------------------------ schedules

#[tokio::test]
async fn schedules_list_filters_and_paginates() {
    let (server, corvo) = setup(
        "GET",
        "/v1/schedules",
        200,
        page(json!([schedule()]), Some("50")),
    )
    .await;
    let r = corvo
        .schedules()
        .list(&ScheduleListParams {
            status: Some(ScheduleStatus::Paused),
            company_id: Some(UUID2.into()),
            limit: Some(10),
            cursor: None,
        })
        .await
        .unwrap();
    assert_eq!(r.data[0].anchor_on, "2026-11-25");
    assert_eq!(r.pagination.next_cursor.as_deref(), Some("50"));
    let q = query(&sent(&server).await.0);
    assert_eq!(q["status"], "paused");
    assert_eq!(q["companyId"], UUID2);
    assert_eq!(q["limit"], "10");
}

#[tokio::test]
async fn schedules_get_update_and_state_changes() {
    let p = format!("/v1/schedules/{UUID}");
    let (_s, corvo) = setup("GET", &p, 200, json!({ "data": schedule() })).await;
    assert_eq!(corvo.schedules().get(UUID).await.unwrap().id, UUID);

    let (server, corvo) = setup("PATCH", &p, 200, json!({ "data": schedule() })).await;
    corvo
        .schedules()
        .update(
            UUID,
            &ScheduleUpdateParams {
                anchor_on: Some("2026-12-01".into()),
                ends_on: Some(None),
                frequency: Some(Frequency::Weekly),
                invoice: Some(json!({ "amount": 20 })),
            },
        )
        .await
        .unwrap();
    assert_eq!(
        sent(&server).await.1,
        json!({ "anchorOn": "2026-12-01", "endsOn": null, "frequency": "WEEKLY",
                "invoice": { "amount": 20 } })
    );

    for action in ["pause", "resume", "cancel"] {
        let (server, corvo) = setup(
            "POST",
            &format!("{p}/{action}"),
            200,
            json!({ "data": schedule() }),
        )
        .await;
        let s = match action {
            "pause" => corvo.schedules().pause(UUID).await,
            "resume" => corvo.schedules().resume(UUID).await,
            _ => corvo.schedules().cancel(UUID).await,
        }
        .unwrap();
        assert_eq!(s.id, UUID);
        assert_eq!(sent(&server).await.0.method.as_str(), "POST");
    }
}

#[tokio::test]
async fn schedules_list_runs() {
    let (server, corvo) = setup(
        "GET",
        &format!("/v1/schedules/{UUID}/runs"),
        200,
        page(
            json!([{ "id": UUID, "attempt": 1, "outcome": "failed", "occurrence_on": "2026-11-25",
                      "ran_on": "2026-11-25", "invoice_id": null, "error_code": "ERROR_X",
                      "error_message": "x", "created_at": "2026-11-25T12:00:00Z" }]),
            None,
        ),
    )
    .await;
    let r = corvo
        .schedules()
        .list_runs(UUID, Some(5), Some("1"))
        .await
        .unwrap();
    assert_eq!(r.data[0].outcome, ScheduleRunOutcome::Failed);
    assert_eq!(query(&sent(&server).await.0)["limit"], "5");
}

#[tokio::test]
async fn schedule_not_found_error_code() {
    let (_s, corvo) = setup(
        "GET",
        &format!("/v1/schedules/{UUID}"),
        404,
        api_error(404, "ERROR_SCHEDULE_NOT_FOUND"),
    )
    .await;
    assert_eq!(
        corvo.schedules().get(UUID).await.unwrap_err().code(),
        "ERROR_SCHEDULE_NOT_FOUND"
    );
}

// -------------------------------------------------------------------- imports

#[tokio::test]
async fn imports_start_sends_multipart() {
    let (server, corvo) = setup(
        "POST",
        "/v1/imports",
        202,
        json!({ "data": { "id": UUID, "status": "processing", "duplicate_of": null } }),
    )
    .await;
    let params = ImportStartParams::new(b"a;b\n1;2".to_vec())
        .file_name("notas.csv")
        .company_id(UUID2);
    let r = corvo.imports().start(&params).await.unwrap();
    assert_eq!(r.status, ImportStatus::Processing);
    let (req, _) = sent(&server).await;
    assert!(header(&req, "content-type")
        .unwrap()
        .starts_with("multipart/form-data; boundary="));
    let body = String::from_utf8_lossy(&req.body);
    assert!(body.contains("name=\"file\"") && body.contains("notas.csv") && body.contains("a;b"));
    assert!(body.contains("name=\"companyId\"") && body.contains(UUID2));
}

#[tokio::test]
async fn imports_list_get_emit_discard() {
    let (server, corvo) = setup("GET", "/v1/imports", 200, json!({ "data": [import_job()] })).await;
    let list = corvo.imports().list(Some(5)).await.unwrap();
    assert_eq!(list[0].counters.valid, 2);
    assert_eq!(query(&sent(&server).await.0)["limit"], "5");

    let p = format!("/v1/imports/{UUID}");
    let (_s, corvo) = setup("GET", &p, 200, json!({ "data": import_job() })).await;
    assert_eq!(
        corvo.imports().get(UUID).await.unwrap().unknown_columns,
        vec!["x"]
    );

    let (server, corvo) = setup(
        "POST",
        &format!("{p}/emit"),
        202,
        json!({ "data": import_job() }),
    )
    .await;
    corvo
        .imports()
        .emit(
            UUID,
            &ImportEmitParams {
                send_email: Some(true),
            },
        )
        .await
        .unwrap();
    assert_eq!(sent(&server).await.1, json!({ "sendEmail": true }));

    let (server, corvo) = setup(
        "POST",
        &format!("{p}/emit"),
        202,
        json!({ "data": import_job() }),
    )
    .await;
    corvo
        .imports()
        .emit(UUID, &ImportEmitParams::default())
        .await
        .unwrap();
    assert!(sent(&server).await.0.body.is_empty());

    let (server, corvo) = setup("DELETE", &p, 204, Value::Null).await;
    corvo.imports().discard(UUID).await.unwrap();
    assert_eq!(sent(&server).await.0.method.as_str(), "DELETE");
}

#[tokio::test]
async fn imports_rows_and_update_row() {
    let row = json!({ "line": 2, "status": "invalid", "data": { "amount": 1 },
        "error_code": "ERROR_IMPORT_X", "error_message": "m", "invoice_id": null });
    let p = format!("/v1/imports/{UUID}/rows");
    let (server, corvo) = setup("GET", &p, 200, page(json!([row.clone()]), Some("50"))).await;
    let r = corvo
        .imports()
        .rows(
            UUID,
            &ImportRowListParams {
                status: Some(ImportRowStatus::Invalid),
                limit: Some(50),
                cursor: Some("0".into()),
            },
        )
        .await
        .unwrap();
    assert_eq!(r.data[0].line, 2);
    let q = query(&sent(&server).await.0);
    assert_eq!(
        (
            q["status"].as_str(),
            q["limit"].as_str(),
            q["cursor"].as_str()
        ),
        ("invalid", "50", "0")
    );

    let (server, corvo) = setup("PATCH", &format!("{p}/2"), 200, json!({ "data": row })).await;
    corvo
        .imports()
        .update_row(UUID, 2, &json!({ "amount": 5 }))
        .await
        .unwrap();
    assert_eq!(sent(&server).await.1, json!({ "item": { "amount": 5 } }));
}

#[tokio::test]
async fn imports_downloads_return_csv_bytes() {
    for (suffix, p) in [
        ("template", "/v1/imports/template".to_owned()),
        ("errors", format!("/v1/imports/{UUID}/errors.csv")),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(p))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "text/csv; charset=utf-8")
                    .set_body_bytes(b"a,b\n".to_vec()),
            )
            .mount(&server)
            .await;
        let corvo = Corvo::new(KEY, server.uri()).unwrap();
        let file = if suffix == "template" {
            corvo.imports().download_template().await
        } else {
            corvo.imports().download_errors(UUID).await
        }
        .unwrap();
        assert_eq!(file.bytes, b"a,b\n");
        assert_eq!(file.mime_type.as_deref(), Some("text/csv"));
    }
}

#[tokio::test]
async fn imports_emit_without_credits_is_402() {
    let (_s, corvo) = setup(
        "POST",
        &format!("/v1/imports/{UUID}/emit"),
        402,
        api_error(402, "ERROR_INSUFFICIENT_CREDITS"),
    )
    .await;
    let err = corvo
        .imports()
        .emit(UUID, &ImportEmitParams::default())
        .await
        .unwrap_err();
    assert_eq!(err.status(), 402);
}

// -------------------------------------------------------- fiscal / tax / dps

#[tokio::test]
async fn fiscal_profiles_list_upsert_approve() {
    let p = format!("/v1/companies/{UUID}/fiscal-profile");
    let (_s, corvo) = setup("GET", &p, 200, json!({ "data": [fiscal_profile()] })).await;
    assert_eq!(
        corvo.fiscal_profiles().list(UUID).await.unwrap()[0].tax_regime,
        FiscalTaxRegime::SimplesNacional
    );

    let (server, corvo) = setup("PUT", &p, 200, json!({ "data": fiscal_profile() })).await;
    let mut params = FiscalProfileUpsertParams::new(FiscalTaxRegime::Mei);
    params.confirm = Some(true);
    params.special_tax_regime = Some(None);
    corvo.fiscal_profiles().upsert(UUID, &params).await.unwrap();
    assert_eq!(
        sent(&server).await.1,
        json!({ "taxRegime": "MEI", "confirm": true, "specialTaxRegime": null })
    );

    let (_s, corvo) = setup(
        "POST",
        &format!("{p}/{UUID2}/approve"),
        201,
        json!({ "data": fiscal_profile() }),
    )
    .await;
    assert_eq!(
        corvo
            .fiscal_profiles()
            .approve(UUID, UUID2)
            .await
            .unwrap()
            .review_status,
        ReviewStatus::Approved
    );
}

#[tokio::test]
async fn service_tax_profiles_crud_and_approve() {
    let p = format!("/v1/companies/{UUID}/service-tax-profiles");
    let (server, corvo) = setup("GET", &p, 200, json!({ "data": [tax_profile()] })).await;
    let l = corvo
        .service_tax_profiles()
        .list(UUID, Some("2026-11-24"))
        .await
        .unwrap();
    assert_eq!(l[0].iss_taxation, Some(IssTaxation::Normal));
    assert_eq!(query(&sent(&server).await.0)["date"], "2026-11-24");

    let (server, corvo) = setup("POST", &p, 201, json!({ "data": tax_profile() })).await;
    corvo
        .service_tax_profiles()
        .create(
            UUID,
            &ServiceTaxProfileCreateParams::new("Perfil", "010101", "2026-01-01"),
        )
        .await
        .unwrap();
    assert_eq!(
        sent(&server).await.1,
        json!({ "name": "Perfil", "serviceCode": "010101", "effectiveFrom": "2026-01-01" })
    );

    let (server, corvo) = setup(
        "PATCH",
        &format!("{p}/{UUID2}"),
        200,
        json!({ "data": tax_profile() }),
    )
    .await;
    corvo
        .service_tax_profiles()
        .update(
            UUID,
            UUID2,
            &ServiceTaxProfileUpdateParams {
                iss_rate: Some("3.00".into()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(sent(&server).await.1, json!({ "issRate": "3.00" }));

    let (server, corvo) = setup(
        "DELETE",
        &format!("{p}/{UUID2}"),
        200,
        json!({ "data": null }),
    )
    .await;
    corvo
        .service_tax_profiles()
        .remove(UUID, UUID2)
        .await
        .unwrap();
    assert_eq!(sent(&server).await.0.method.as_str(), "DELETE");

    let (_s, corvo) = setup(
        "POST",
        &format!("{p}/{UUID2}/approve"),
        201,
        json!({ "data": tax_profile() }),
    )
    .await;
    corvo
        .service_tax_profiles()
        .approve(UUID, UUID2)
        .await
        .unwrap();
}

#[tokio::test]
async fn dps_sequence_get_set_and_below_current() {
    let p = format!("/v1/companies/{UUID}/dps-sequence");
    let (_s, corvo) = setup(
        "GET",
        &p,
        200,
        json!({ "data": { "lastNumber": 10, "series": "1" } }),
    )
    .await;
    assert_eq!(
        corvo.dps_sequence().get(UUID).await.unwrap().last_number,
        10
    );

    let (server, corvo) = setup(
        "PUT",
        &p,
        200,
        json!({ "data": { "lastNumber": 99, "series": "1" } }),
    )
    .await;
    let r = corvo
        .dps_sequence()
        .set(
            UUID,
            &DpsSequenceSetParams {
                last_number: Some(99),
            },
        )
        .await
        .unwrap();
    assert_eq!(r.last_number, 99);
    assert_eq!(sent(&server).await.1, json!({ "lastNumber": 99 }));

    let (_s, corvo) = setup("PUT", &p, 422, api_error(422, "DPS_NUMBER_BELOW_CURRENT")).await;
    let err = corvo
        .dps_sequence()
        .set(
            UUID,
            &DpsSequenceSetParams {
                last_number: Some(1),
            },
        )
        .await
        .unwrap_err();
    assert_eq!(err.code(), "DPS_NUMBER_BELOW_CURRENT");
}

// --------------------------------------------------------------- integrations

#[tokio::test]
async fn integrations_providers_and_connections() {
    let (_s, corvo) = setup(
        "GET",
        "/v1/integrations/providers",
        200,
        json!({ "data": [{ "id": "stripe", "name": "Stripe", "version": "1", "supportedFlows": ["sale"],
            "credentialFields": [{ "key": "apiKey", "label": "API key", "secret": true, "revealOnce": false }],
            "webhookInstructions": { "pt": "p", "en": "e" } }] }),
    )
    .await;
    let providers = corvo.integrations().list_providers().await.unwrap();
    assert_eq!(providers[0].credential_fields[0].key, "apiKey");

    let (_s, corvo) = setup(
        "GET",
        "/v1/integrations/connections",
        200,
        json!({ "data": [connection()] }),
    )
    .await;
    assert_eq!(
        corvo.integration_connections().list().await.unwrap()[0].state,
        ConnectionState::Draft
    );

    let mut created = connection();
    created["secrets"] = json!({ "webhookSecret": "s3" });
    let (server, corvo) = setup(
        "POST",
        "/v1/integrations/connections",
        201,
        json!({ "data": created }),
    )
    .await;
    let mut creds = BTreeMap::new();
    creds.insert("apiKey".to_owned(), "k".to_owned());
    let c = corvo
        .integration_connections()
        .create(&IntegrationConnectionCreateParams {
            company_id: UUID2.into(),
            provider: "stripe".into(),
            credentials: creds,
            environment: Some(Environment::Test),
            active_from: None,
        })
        .await
        .unwrap();
    assert_eq!(c.secrets["webhookSecret"], "s3");
    assert_eq!(c.connection.provider, "stripe");
    assert_eq!(
        sent(&server).await.1,
        json!({ "companyId": UUID2, "provider": "stripe", "credentials": { "apiKey": "k" }, "environment": "test" })
    );
}

#[tokio::test]
async fn integration_connection_actions() {
    let p = format!("/v1/integrations/connections/{UUID}");
    let (_s, corvo) = setup("GET", &p, 200, json!({ "data": connection() })).await;
    assert_eq!(
        corvo.integration_connections().get(UUID).await.unwrap().id,
        UUID
    );

    for action in ["validate", "activate", "pause", "disconnect"] {
        let (server, corvo) = setup(
            "POST",
            &format!("{p}/{action}"),
            200,
            json!({ "data": connection() }),
        )
        .await;
        let c = corvo.integration_connections();
        match action {
            "validate" => c.validate(UUID).await,
            "activate" => c.activate(UUID).await,
            "pause" => c.pause(UUID).await,
            _ => c.disconnect(UUID).await,
        }
        .unwrap();
        assert_eq!(sent(&server).await.0.method.as_str(), "POST");
    }

    let (server, corvo) = setup(
        "PUT",
        &format!("{p}/credentials"),
        200,
        json!({ "data": connection() }),
    )
    .await;
    let mut creds = BTreeMap::new();
    creds.insert("apiKey".to_owned(), "novo".to_owned());
    corvo
        .integration_connections()
        .update_credentials(UUID, &creds)
        .await
        .unwrap();
    assert_eq!(
        sent(&server).await.1,
        json!({ "credentials": { "apiKey": "novo" } })
    );
}

#[tokio::test]
async fn integration_rules_events_operations() {
    let p = format!("/v1/integrations/connections/{UUID}");
    let rule = json!({ "id": UUID, "allowAnonymousCustomer": false, "cancelOnRefund": true,
        "externalProductId": null, "sendEmail": true, "service": { "code": "010101" } });
    let (_s, corvo) = setup(
        "GET",
        &format!("{p}/rules"),
        200,
        json!({ "data": [rule.clone()] }),
    )
    .await;
    assert!(
        corvo
            .integration_connections()
            .list_rules(UUID)
            .await
            .unwrap()[0]
            .cancel_on_refund
    );

    let (server, corvo) = setup("PUT", &format!("{p}/rules"), 200, json!({ "data": [rule] })).await;
    corvo
        .integration_connections()
        .replace_rules(
            UUID,
            &[IntegrationRuleInput {
                send_email: true,
                service: InvoiceServiceInput {
                    code: Some("010101".into()),
                    ..Default::default()
                },
                allow_anonymous_customer: None,
                cancel_on_refund: Some(true),
                external_product_id: None,
            }],
        )
        .await
        .unwrap();
    assert_eq!(
        sent(&server).await.1,
        json!({ "rules": [{ "sendEmail": true, "service": { "code": "010101" }, "cancelOnRefund": true }] })
    );

    let (server, corvo) = setup(
        "GET",
        &format!("{p}/events"),
        200,
        json!({ "data": [{ "id": UUID, "attempts": 1, "deliveryId": "d", "externalType": "charge",
            "lastError": null, "operationId": null, "state": "needs_attention",
            "createdAt": "2026-11-24T00:00:00Z" }] }),
    )
    .await;
    let ev = corvo
        .integration_connections()
        .list_events(UUID, Some(10))
        .await
        .unwrap();
    assert_eq!(ev[0].state, EventState::NeedsAttention);
    assert_eq!(query(&sent(&server).await.0)["limit"], "10");

    let (_s, corvo) = setup(
        "GET",
        &format!("{p}/operations"),
        200,
        json!({ "data": [operation()] }),
    )
    .await;
    let ops = corvo
        .integration_connections()
        .list_operations(UUID, None)
        .await
        .unwrap();
    assert_eq!(ops[0].fiscal_status, OperationFiscalStatus::Blocked);
}

#[tokio::test]
async fn integration_operations_complete_customer_and_retry() {
    let p = format!("/v1/integrations/operations/{UUID}");
    let (server, corvo) = setup(
        "PATCH",
        &format!("{p}/customer"),
        200,
        json!({ "data": operation() }),
    )
    .await;
    corvo
        .integration_operations()
        .complete_customer(
            UUID,
            &CompleteCustomerParams {
                tax_id: "52998224725".into(),
                email: None,
                name: Some("Maria".into()),
            },
        )
        .await
        .unwrap();
    assert_eq!(
        sent(&server).await.1,
        json!({ "taxId": "52998224725", "name": "Maria" })
    );

    let (_s, corvo) = setup(
        "POST",
        &format!("{p}/retry"),
        200,
        json!({ "data": { "state": "processed", "operationId": UUID } }),
    )
    .await;
    assert_eq!(
        corvo
            .integration_operations()
            .retry(UUID)
            .await
            .unwrap()
            .state,
        OperationRetryState::Processed
    );
}

// ----------------------------------------------- credits, portal, health, etc.

#[tokio::test]
async fn credits_preview_purchase_is_a_get_with_units() {
    let (server, corvo) = setup(
        "GET",
        "/v1/credits/purchase/preview",
        200,
        json!({ "data": { "units": 10, "unitPriceCents": 99, "totalCents": 990 } }),
    )
    .await;
    let p = corvo.credits().preview_purchase(10).await.unwrap();
    assert_eq!((p.units, p.unit_price_cents, p.total_cents), (10, 99, 990));
    assert_eq!(query(&sent(&server).await.0)["units"], "10");
}

#[tokio::test]
async fn fiscal_portal_status() {
    let (_s, corvo) = setup(
        "GET",
        "/v1/fiscal-portal/status",
        200,
        json!({ "data": { "available": false, "environment": "test",
            "retryAt": "2026-11-24T12:00:00Z", "since": "2026-11-24T11:00:00Z" } }),
    )
    .await;
    let s = corvo.fiscal_portal().status().await.unwrap();
    assert!(!s.available);
    assert_eq!(s.retry_at.as_deref(), Some("2026-11-24T12:00:00Z"));
}

#[tokio::test]
async fn no_plan_assigned_is_an_error_code() {
    let (_s, corvo) = setup(
        "GET",
        "/v1/billing/plan",
        404,
        api_error(404, "NO_PLAN_ASSIGNED"),
    )
    .await;
    assert_eq!(
        corvo.billing().plan().await.unwrap_err().code(),
        "NO_PLAN_ASSIGNED"
    );
}

#[tokio::test]
async fn webhook_changes_environment_and_test_result() {
    let (server, corvo) = setup("GET", "/v1/webhooks/deliveries", 200, page(json!([]), None)).await;
    corvo
        .webhooks()
        .deliveries()
        .list(WebhookDeliveryListParams {
            hours: Some(0.5),
            environment: Some(Environment::Live),
            ..Default::default()
        })
        .await
        .unwrap();
    let q = query(&sent(&server).await.0);
    assert_eq!(
        (q["hours"].as_str(), q["environment"].as_str()),
        ("0.5", "live")
    );

    let (_s, corvo) = setup(
        "POST",
        &format!("/v1/webhooks/{UUID}/test"),
        200,
        json!({ "data": { "sent": false, "error": "refused", "statusCode": 500 } }),
    )
    .await;
    let r = corvo.webhooks().test(UUID).await.unwrap();
    assert!(!r.sent);
    assert_eq!(r.status_code, Some(500));

    let (_s, corvo) = setup(
        "GET",
        "/v1/webhooks",
        200,
        page(
            json!([{ "id": UUID, "url": "https://x.com/h", "events": ["nfse.invoice.taker_rejected"],
                "active": true, "environment": "test", "createdAt": "a", "updatedAt": "b" }]),
            None,
        ),
    )
    .await;
    let w = corvo.webhooks().list(Default::default()).await.unwrap();
    assert_eq!(w.data[0].environment, Some(Environment::Test));
}

#[tokio::test]
async fn health_live_has_no_envelope() {
    let (_s, corvo) = setup(
        "GET",
        "/v1/health/live",
        200,
        json!({ "status": "ok", "timestamp": "2026-11-24T00:00:00Z", "uptime": 12.3 }),
    )
    .await;
    assert_eq!(corvo.health().live().await.unwrap()["status"], "ok");
}

#[tokio::test]
async fn user_agent_carries_the_sdk_version() {
    let (server, corvo) = setup(
        "GET",
        "/v1/fiscal-portal/status",
        200,
        json!({ "data": {
        "available": true, "environment": "live" } }),
    )
    .await;
    corvo.fiscal_portal().status().await.unwrap();
    let ua = header(&sent(&server).await.0, "user-agent").unwrap();
    assert_eq!(ua, format!("corvo-nfse-rust/{}", VERSION));
    assert_eq!(VERSION, "0.3.0");
}
