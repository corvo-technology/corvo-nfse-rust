# corvo-nfse-rust

**The Rust SDK for the Corvo NFS-e API — built for self-hosted installs, `Result`-based errors, and pagination you don't have to think about.**

Issue, list, download, and reconcile Brazilian electronic service invoices (NFS-e) from a Rust 1.75+ service. Async (`tokio` + `reqwest` with `rustls` — no OpenSSL), idiomatic `Result<T, CorvoError>`, and zero panics for API, network, or timeout failures.

## Why corvo-nfse-rust

- **Self-hosted first.** Point it at your own Corvo instance with a base URL; the default `https://api.corvo.tech` only applies when you don't.
- **`rustls`, not OpenSSL.** TLS comes from `rustls` via `reqwest`, so builds are hermetic everywhere.
- **Errors that never panic.** API, network, and timeout failures are `Err(CorvoError::…)` values carrying the server's stable `code`, HTTP `status`, `request_id`, and field-level `details`. Match on them; propagate them with `?`.
- **Pagination you don't have to think about.** Every list returns `Paginated<T>` (`{ data, pagination }`). Need every page? `.iterate(params).collect()` walks them for you; page failures surface as the first `Err`.

## Install

```toml
[dependencies]
corvo-nfse = "0.1"
tokio = { version = "1", features = ["full"] }
```

## SDK quickstart

```rust
use corvo_nfse::Corvo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let corvo = Corvo::builder()
        .api_key("corvo_live_xxx")          // or set CORVO_API_KEY
        .base_url("https://api.corvo.tech") // or CORVO_BASE_URL for self-hosted
        .build()?;

    let page = corvo.companies().list(Default::default()).await?;
    println!("{} companies", page.data.len());
    Ok(())
}
```

### Issue an invoice

```rust
use corvo_nfse::{InvoiceCreateParams, InvoiceCustomerInput, InvoiceServiceInput};

let invoice = corvo
    .invoices()
    .create(
        &InvoiceCreateParams {
            company_id: Some("comp_…".into()),
            customer: InvoiceCustomerInput {
                name: "Customer".into(),
                tax_id: "52998224725".into(),
                email: None,
                address: None,
            },
            service: InvoiceServiceInput {
                code: "0107".into(),
                municipality_code: Some("3550308".into()),
                description: Some("Development services".into()),
            },
            amount: 199.90,
            competence_date: "2026-10-13".into(),
            email: None,
            options: None,
            company_tax_id: None,
            external_id: None,
            correlation_id: None,
        },
        Some("order-42"), // optional idempotency key
    )
    .await?;
println!("issued {}", invoice.id);
```

### Error handling

```rust
use corvo_nfse::{Corvo, CorvoError};

match corvo.invoices().get("inv_…").await {
    Ok(invoice) => println!("{invoice:?}"),
    Err(CorvoError::Api { status, code, details, request_id, .. }) => {
        eprintln!("API error {status} ({code}), request {request_id:?}: {details:?}");
    }
    Err(e @ CorvoError::Timeout) => eprintln!("retry later: {e}"),
    Err(e) => eprintln!("fatal: {e}"),
}
```

`CorvoError` variants: `Api` (with `status`, `code`, `message`, `details`, `request_id`, `retry_after`), `Network`, `Timeout`, `InvalidUrl`, `Config`. Helpers: `.status()` (0 for network/timeout), `.code()` (stable, `network_error`/`timeout` for local failures), `.request_id()`, `.retry_after()`.

### Iterate every page

```rust
let invoices = corvo.invoices().iterate(&Default::default());
let all: Vec<_> = invoices.collect().await?; // first page failure → Err
```

### Download PDF / XML

```rust
let pdf = corvo.invoices().pdf("inv_…").await?;
std::fs::write(pdf.file_name.as_deref().unwrap_or("invoice.pdf"), &pdf.bytes)?;
```

### Verify webhook signatures

```rust
use corvo_nfse::verify_signature;

// raw_body: the request body bytes, exactly as received — do not re-serialize.
if verify_signature(&raw_body, signature_header, "whsec_…") {
    // authentic
}
```

HMAC-SHA256, constant-time comparison, shared known-answer vector across all Corvo SDKs.

## Every operation

| Namespace | Operations |
|---|---|
| `corvo.companies()` | `create`, `list`, `iterate`, `get`, `update`, `remove` |
| `corvo.certificates()` | `upload` (multipart built for you), `list`, `remove` |
| `corvo.invoices()` | `create`, `get`, `list`, `iterate`, `events`, `pdf`, `xml`, `download_url`, `send_email` |
| `corvo.webhooks()` | `list`, `create`, `remove`, `test`, `.deliveries().{list, iterate, stats, retry}` |
| `corvo.credits()` | `balance`, `plans`, `history`, `iterate_history` |
| `corvo.billing()` | `plan`, `usage` |
| `corvo.health()` | `live`, `ready` (no API key required) |

28 API-key operations plus 2 health checks — the exact surface of the Node SDK.

## Configuration

| Option | Environment | Default |
|---|---|---|
| `api_key` | `CORVO_API_KEY` | required |
| `base_url` | `CORVO_BASE_URL` | `https://api.corvo.tech` |
| `prefix` | — | `v1` |
| `timeout` | — | 30s |
| `allow_insecure_http` | — | `false` (`http://` allowed only for loopback/private hosts) |

## CLI

This crate is a library — no CLI binary. For the CLI, use the Node SDK (`corvo-nfse-node`).

## Development

```sh
cargo build
cargo test              # unit + mock-HTTP integration tests; contract test auto-skips
CORVO_API_KEY=… CORVO_BASE_URL=… cargo test --test contract   # live contract test
```

## License

MIT © Corvo Technology
