//! Resource-level tests for the remaining namespaces: certificates (multipart),
//! webhooks, credits, billing (snake_case wire format), and health (unwrap none).

use corvo_nfse::{
    CertificateUploadParams, Corvo, Environment, WebhookCreateParams, WebhookDeliveryListParams,
    WEBHOOK_EVENTS,
};
use serde_json::json;
use wiremock::matchers::{body_partial_json, method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

fn client_for(server: &MockServer) -> Corvo {
    Corvo::builder()
        .api_key("corvo_test_0123456789abcdef0123456789abcdef01234567")
        .base_url(server.uri())
        .build()
        .expect("test client")
}

fn certificate_json(id: &str) -> serde_json::Value {
    json!({
        "id": id,
        "companyId": "c_1",
        "serialNumber": "01",
        "issuer": "AC",
        "subject": "CN=Acme",
        "fingerprint": "ab:cd",
        "validFrom": "2026-01-01T00:00:00Z",
        "validUntil": "2027-01-01T00:00:00Z",
        "status": "active",
        "createdAt": "2026-10-13T00:00:00Z"
    })
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
async fn certificate_upload_sends_multipart_with_file_and_password() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/companies/c_1/certificates"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": certificate_json("cert_1")
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let params = CertificateUploadParams::new(b"fake-p12-bytes".to_vec(), "s3cret");
    let cert = corvo
        .certificates()
        .upload("c_1", &params)
        .await
        .expect("upload");
    assert_eq!(cert.id, "cert_1");

    let requests: Vec<Request> = server.received_requests().await.unwrap();
    let request = requests.last().expect("one request");
    let content_type = request
        .headers
        .get("content-type")
        .expect("content-type")
        .to_str()
        .unwrap()
        .to_owned();
    assert!(
        content_type.starts_with("multipart/form-data; boundary="),
        "got {content_type}"
    );
    let body = String::from_utf8_lossy(&request.body);
    assert!(body.contains("fake-p12-bytes"));
    assert!(body.contains("password"));
    assert!(body.contains("s3cret"));
    assert!(body.contains("certificate.p12"));
}

#[tokio::test]
async fn certificate_upload_uses_explicit_file_name() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/companies/c_1/certificates"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": certificate_json("cert_1")
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let params = CertificateUploadParams::new(b"p12".to_vec(), "pw").file_name("acme.p12");
    corvo
        .certificates()
        .upload("c_1", &params)
        .await
        .expect("upload");

    let requests = server.received_requests().await.unwrap();
    let body = String::from_utf8_lossy(&requests.last().unwrap().body);
    assert!(body.contains("acme.p12"));
}

#[tokio::test]
async fn webhook_create_returns_secret() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/webhooks"))
        .and(body_partial_json(json!({
            "url": "https://example.com/hook",
            "events": ["nfse.invoice.authorized"]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {
                "id": "wh_1",
                "url": "https://example.com/hook",
                "events": ["nfse.invoice.authorized"],
                "active": true,
                "createdAt": "2026-10-13T00:00:00Z",
                "updatedAt": "2026-10-13T00:00:00Z",
                "secret": "whsec_test_123"
            }
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let params = WebhookCreateParams {
        url: "https://example.com/hook".into(),
        events: vec![WEBHOOK_EVENTS[0].to_owned()],
    };
    let webhook = corvo.webhooks().create(&params).await.expect("create");
    assert_eq!(webhook.secret, "whsec_test_123");
}

#[tokio::test]
async fn delivery_stats_sends_query_params() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/webhooks/deliveries/stats"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": { "total": 9, "sent": 7, "failed": 1, "pending": 1, "delayed": 0 }
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let stats = corvo
        .webhooks()
        .deliveries()
        .stats(Some(24), Some(Environment::Test))
        .await
        .expect("stats");
    assert_eq!(stats.total, 9);

    let requests = server.received_requests().await.unwrap();
    let url = requests.last().unwrap().url.to_string();
    assert!(url.contains("hours=24"), "got {url}");
    assert!(url.contains("environment=test"), "got {url}");
}

#[tokio::test]
async fn credit_balance_unwraps_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/credits"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {
                "tenantId": "ten_1",
                "balance": 100.5,
                "overageUnits": 0,
                "overageAmountCents": 0,
                "hardLimit": null
            }
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let balance = corvo.credits().balance().await.expect("balance");
    assert_eq!(balance.balance, 100.5);
    assert_eq!(balance.hard_limit, None);
}

#[tokio::test]
async fn credit_plans_returns_the_items() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/credits/plans"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [ {
                "id": "plan_1", "slug": "pro", "name": "Pro", "description": "pro plan",
                "monthlyCreditAllowance": 750, "dailyLimit": null, "maxActiveCompanies": 5, "creditUnitPriceCents": 9,
                "throughputRps": 10, "includesWebhooks": true, "includesXmlDownload": true,
                "includesPdfDownload": true, "includesApiKeys": true, "supportTier": "priority",
                "active": true, "createdAt": "2026-10-13T00:00:00Z",
                "updatedAt": "2026-10-13T00:00:00Z"
            } ],
            "pagination": { "limit": 50, "hasMore": false, "nextCursor": null }
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let plans = corvo
        .credits()
        .plans(Default::default())
        .await
        .expect("plans");
    assert_eq!(plans.data.len(), 1);
    assert_eq!(plans.data[0].slug, "pro");
}

#[tokio::test]
async fn billing_plan_uses_snake_case_wire_format() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/billing/plan"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {
                "plan": {
                    "slug": "pro", "name": "Pro", "monthly_allowance": 1000,
                    "daily_limit": 100, "max_active_companies": 5, "monthly_price_cents": 9900
                },
                "subscription": {
                    "status": "active", "paid_until": "2026-11-01T00:00:00Z",
                    "cancel_at_period_end": false, "scheduled_plan": null, "blocked": false
                },
                "usage": {
                    "overage_units": 0, "overage_amount_cents": 0, "hard_limit": null,
                    "credits_used": 10, "credits_reserved": 0, "credits_remaining": 990,
                    "monthly_allowance": 1000, "daily_usage": 3, "daily_limit": 100,
                    "active_companies": 1, "max_active_companies": 5,
                    "period_start": "2026-10-01T00:00:00Z", "period_end": "2026-11-01T00:00:00Z"
                }
            }
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let plan = corvo.billing().plan().await.expect("plan");
    assert_eq!(plan.plan.slug, "pro");
    assert_eq!(plan.plan.monthly_allowance, 1000);
    assert_eq!(plan.subscription.status, "active");
    assert_eq!(plan.usage.credits_remaining, 990);
}

#[tokio::test]
async fn billing_usage_is_camel_case() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/billing/usage"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {
                "canEmit": true, "creditsRemaining": 42, "creditsUsed": 8,
                "creditsReserved": 0, "monthlyAllowance": 1000, "dailyUsage": 3,
                "dailyLimit": null, "activeCompanies": 1, "maxActiveCompanies": 5,
                "periodStart": null, "periodEnd": null
            }
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let quota = corvo.billing().usage().await.expect("usage");
    assert!(quota.can_emit);
    assert_eq!(quota.credits_remaining, 42);
}

#[tokio::test]
async fn health_checks_do_not_unwrap_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/health/live"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "status": "ok" })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let live = corvo.health().live().await.expect("live");
    assert_eq!(live["status"], "ok");
}

#[tokio::test]
async fn delivery_retry_posts_and_returns_retried_flag() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/webhooks/deliveries/del_1/retry"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": { "retried": true }
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let result = corvo
        .webhooks()
        .deliveries()
        .retry("del_1")
        .await
        .expect("retry");
    assert!(result.retried);
}

#[tokio::test]
async fn delivery_list_iterate_unwraps_plain_items() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/webhooks/deliveries"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{
                "id": "del_1", "tenantId": "ten_1", "webhookConfigId": "wh_1",
                "eventType": "nfse.invoice.authorized", "environment": "test",
                "url": "https://example.com/hook", "payload": {},
                "statusCode": 200, "responseBody": null, "errorMessage": null,
                "status": "delivered", "durationMs": 12, "retryCount": 0,
                "nextRetryAt": null, "lastError": null,
                "createdAt": "2026-10-13T00:00:00Z"
            }],
            "pagination": { "limit": 50, "hasMore": false, "nextCursor": null }
        })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let deliveries = corvo
        .webhooks()
        .deliveries()
        .list(WebhookDeliveryListParams::default())
        .await
        .expect("deliveries");
    assert_eq!(deliveries.data.len(), 1);
    assert_eq!(deliveries.data[0].id, "del_1");
}

#[tokio::test]
async fn company_update_sends_patch() {
    let server = MockServer::start().await;
    let mut updated = company_json("c_1");
    updated["legalName"] = json!("Acme 2");
    Mock::given(method("PATCH"))
        .and(path("/v1/companies/c_1"))
        .and(body_partial_json(json!({ "legalName": "Acme 2" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "data": updated })))
        .mount(&server)
        .await;

    let corvo = client_for(&server);
    let update = corvo_nfse::CompanyUpdateParams {
        legal_name: Some("Acme 2".into()),
        ..Default::default()
    };
    let company = corvo
        .companies()
        .update("c_1", &update)
        .await
        .expect("update");
    assert_eq!(company.legal_name, "Acme 2");
}
