//! Webhook signature verification — the shared HMAC-SHA256 known vector.

use corvo_nfse::{verify_signature, WEBHOOK_EVENTS};

const BODY: &str = r#"{"type":"nfse.invoice.authorized","data":{"id":"inv_1"}}"#;
const SECRET: &str = "whsec_test_123";
const SIGNATURE: &str = "sha256=8f9bd040d8f717b19618ab3395ddfa4b8c176b8dc863195227b88dd85426bbbc";

#[test]
fn accepts_known_hmac_vector() {
    assert!(verify_signature(BODY, Some(SIGNATURE), SECRET));
    assert!(verify_signature(BODY.as_bytes(), Some(SIGNATURE), SECRET));
}

#[test]
fn accepts_header_with_whitespace() {
    assert!(verify_signature(
        BODY,
        Some("  sha256=8f9bd040d8f717b19618ab3395ddfa4b8c176b8dc863195227b88dd85426bbbc  "),
        SECRET
    ));
}

#[test]
fn rejects_tampered_body() {
    let tampered = BODY.replace("inv_1", "inv_2");
    assert!(!verify_signature(
        tampered.as_str(),
        Some(SIGNATURE),
        SECRET
    ));
}

#[test]
fn rejects_wrong_secret() {
    assert!(!verify_signature(BODY, Some(SIGNATURE), "whsec_other"));
}

#[test]
fn rejects_malformed_headers() {
    assert!(!verify_signature(BODY, None, SECRET));
    assert!(!verify_signature(BODY, Some(""), SECRET));
    // Missing the sha256= prefix.
    assert!(!verify_signature(
        BODY,
        Some("8f9bd040d8f717b19618ab3395ddfa4b8c176b8dc863195227b88dd85426bbbc"),
        SECRET
    ));
    assert!(!verify_signature(BODY, Some("sha256=zzzz"), SECRET));
    // Empty secret.
    assert!(!verify_signature(BODY, Some(SIGNATURE), ""));
}

#[test]
fn webhook_events_list_is_complete() {
    assert_eq!(WEBHOOK_EVENTS.len(), 9);
    assert!(WEBHOOK_EVENTS.contains(&"nfse.invoice.fiscal_discrepancy"));
    assert!(!WEBHOOK_EVENTS.contains(&"webhook.test"));
    assert!(WEBHOOK_EVENTS.contains(&"nfse.invoice.authorized"));
}
