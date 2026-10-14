//! Contract test against a live Corvo instance.
//!
//! Skipped unless BOTH `CORVO_API_KEY` and `CORVO_BASE_URL` are set:
//!
//! ```sh
//! CORVO_API_KEY=corvo_test_... CORVO_BASE_URL=http://localhost:3000 cargo test --test contract
//! ```

use corvo_nfse::Corvo;

#[tokio::test]
async fn health_and_company_page_against_live_instance() {
    let enabled = std::env::var("CORVO_API_KEY").is_ok() && std::env::var("CORVO_BASE_URL").is_ok();
    if !enabled {
        eprintln!("skipping contract test: CORVO_API_KEY / CORVO_BASE_URL not both set");
        return;
    }

    let corvo = Corvo::from_env().expect("client from env");

    let live = corvo.health().live().await.expect("health.live");
    assert_eq!(live["status"], "ok");

    let page = corvo
        .companies()
        .list(corvo_nfse::CompanyListParams {
            limit: Some(1),
            cursor: None,
        })
        .await
        .expect("companies.list");
    assert_eq!(page.pagination.limit, 1);
    assert!(page.data.len() <= 1);
}
