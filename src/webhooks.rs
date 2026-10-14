//! Webhook signature verification (HMAC-SHA256, constant-time compare).

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Verify a `sha256=<hex>` webhook signature against the raw body and secret.
///
/// Accepts the raw request body exactly as received (no re-serialization) and
/// the `X-Corvo-Signature` header value. Returns `false` — never panics — for
/// missing/malformed headers, an empty secret, or a mismatched signature.
/// Comparison is constant-time.
///
/// ```
/// use corvo_nfse::verify_signature;
///
/// let body = r#"{"type":"nfse.invoice.authorized","data":{"id":"inv_1"}}"#;
/// let header = "sha256=8f9bd040d8f717b19618ab3395ddfa4b8c176b8dc863195227b88dd85426bbbc";
/// assert!(verify_signature(body, Some(header), "whsec_test_123"));
/// assert!(!verify_signature(body, Some(header), "whsec_other"));
/// ```
pub fn verify_signature(
    raw_body: impl AsRef<[u8]>,
    signature_header: Option<&str>,
    secret: &str,
) -> bool {
    if secret.is_empty() {
        return false;
    }
    let Some(header) = signature_header else {
        return false;
    };
    let Some(hex) = header.trim().strip_prefix("sha256=") else {
        return false;
    };
    if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()) {
        return false;
    }
    let Some(received) = decode_hex(hex) else {
        return false;
    };
    let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
        Ok(mac) => mac,
        // HMAC accepts keys of any length; this is unreachable in practice.
        Err(_) => return false,
    };
    mac.update(raw_body.as_ref());
    // verify_slice performs a constant-time comparison of the tags.
    mac.verify_slice(&received).is_ok()
}

/// Decode an ASCII hex string into bytes; `None` on odd length or bad digits.
fn decode_hex(hex: &str) -> Option<Vec<u8>> {
    let bytes = hex.as_bytes();
    if bytes.len() % 2 != 0 {
        return None;
    }
    bytes
        .chunks_exact(2)
        .map(|pair| (hex_val(pair[0])? << 4) | hex_val(pair[1])?)
        .collect::<Option<Vec<u8>>>()
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &str = r#"{"type":"nfse.invoice.authorized","data":{"id":"inv_1"}}"#;
    const SECRET: &str = "whsec_test_123";
    const SIGNATURE: &str =
        "sha256=8f9bd040d8f717b19618ab3395ddfa4b8c176b8dc863195227b88dd85426bbbc";

    #[test]
    fn accepts_known_vector() {
        assert!(verify_signature(BODY, Some(SIGNATURE), SECRET));
        assert!(verify_signature(BODY.as_bytes(), Some(SIGNATURE), SECRET));
    }

    #[test]
    fn rejects_tampered_body() {
        assert!(!verify_signature(
            BODY.replace("inv_1", "inv_2").as_str(),
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
        assert!(!verify_signature(
            BODY,
            Some("8f9bd040d8f717b19618ab3395ddfa4b8c176b8dc863195227b88dd85426bbbc"),
            SECRET
        ));
        assert!(!verify_signature(BODY, Some("sha256=zzzz"), SECRET));
        assert!(!verify_signature(BODY, Some(SIGNATURE), ""));
    }
}
