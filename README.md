# corvo-nfse

**The Rust SDK for the [Corvo](https://corvo.tech) NFS-e API.** Register companies, calculate taxes, issue Brazilian service invoices (NFS-e), follow them by webhook and download the PDF and XML, from Rust 1.75+.

- **Async** on `tokio` + `reqwest` with `rustls` (no OpenSSL).
- **`Result`-based errors.** API, network and timeout failures are `Err(CorvoError::…)` values carrying the server's stable `code`, the HTTP `status`, the `request_id` and the field-level `details`. Nothing panics.
- **Pagination built in.** Every list returns `Paginated<T>` (`{ data, pagination }`), and `.iterate(params)` streams every page.
- **Safe retries.** Issuing an invoice and sending an email accept an idempotency key.
- **Webhook signature check** in constant time.

Full API reference: <https://corvo.tech/en/docs>.

## Install

```toml
[dependencies]
corvo-nfse = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quickstart

Start with a `corvo_test_...` key: invoices issued with it have no fiscal value.

```rust
use corvo_nfse::Corvo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let corvo = Corvo::builder()
        .api_key("corvo_test_...") // or set CORVO_API_KEY
        // .base_url("https://api.corvo.tech") // default; also read from CORVO_BASE_URL
        .build()?;

    let page = corvo.companies().list(Default::default()).await?;
    println!("{} companies", page.data.len());
    Ok(())
}
```

### Register a company

```rust
use corvo_nfse::{CompanyCreateParams, TaxRegime};

let company = corvo
    .companies()
    .create(&CompanyCreateParams {
        legal_name: "Empresa Exemplo Ltda".into(),
        tax_id: "11222333000181".into(),
        tax_regime: TaxRegime::SimplesNacional,
        municipality_code: "3550308".into(), // IBGE code, 7 digits
        default_service_code: Some("010101".into()), // used when an invoice omits service.code
        trade_name: None,
        municipal_reg: None,
        send_municipal_reg: None,
        simple_national: None,
        special_regime: None,
        address: None,
    })
    .await?;
```

In `live`, also upload the A1 certificate with `corvo.certificates().upload(...)`. The company is ready when `company.status` is `"ready"` and `company.setup_gaps` is empty.

### Estimate the taxes before issuing

Nothing is issued, stored or charged.

```rust
use corvo_nfse::{InvoiceCreateParams, InvoiceServiceInput};

let estimate = corvo
    .invoices()
    .calculate(&InvoiceCreateParams {
        company_tax_id: Some("11222333000181".into()),
        service: InvoiceServiceInput {
            description: Some("Consultoria".into()),
            ..Default::default()
        },
        amount: 1500.0,
        competence_date: "2026-10-16".into(),
        ..Default::default()
    })
    .await?;
// estimate.status is "ready" or "blocked": a fiscal gap is data, not an error.
```

### Issue an invoice

Issuing is asynchronous: a successful call means the request was accepted, not that the invoice is authorized. Follow the status with `invoices().get()` or webhooks.

```rust
use corvo_nfse::{InvoiceCreateParams, InvoiceCustomerInput, InvoiceServiceInput};

let invoice = corvo
    .invoices()
    .create(
        &InvoiceCreateParams {
            company_tax_id: Some("11222333000181".into()), // or company_id
            // The customer is optional; when sent, tax_id (CPF or CNPJ) is required.
            customer: Some(InvoiceCustomerInput {
                tax_id: "52998224725".into(),
                name: Some("Cliente".into()),
                email: Some("cliente@example.com".into()),
                ..Default::default()
            }),
            service: InvoiceServiceInput {
                description: Some("Consultoria".into()), // code and city default to the company's
                ..Default::default()
            },
            amount: 150.50,
            competence_date: "2026-10-16".into(),
            ..Default::default()
        },
        Some("order-42"), // idempotency key: the same key never issues twice
    )
    .await?;
println!("queued {}", invoice.id);
```

Set `options: Some(InvoiceOptions { invoice_type: Some(InvoiceType::Schedule), ..Default::default() })` with a future `competence_date` to schedule the issuance.

### Error handling

```rust
use corvo_nfse::CorvoError;

match corvo.invoices().get("inv_…").await {
    Ok(invoice) => println!("{invoice:?}"),
    Err(CorvoError::Api { status, code, details, request_id, .. }) => {
        eprintln!("API error {status} ({code}), request {request_id:?}: {details:?}");
    }
    Err(e @ CorvoError::Timeout) => eprintln!("retry later: {e}"),
    Err(e) => eprintln!("fatal: {e}"),
}
```

`CorvoError` variants: `Api` (with `status`, `code`, `message`, `details`, `request_id`, `retry_after`), `Network`, `Timeout`, `InvalidUrl`, `Config`.

### Iterate every page

```rust
let all: Vec<_> = corvo.invoices().iterate(&Default::default()).collect().await?; // a failed page is the first Err
```

### Download the PDF or XML

The invoice must be `authorized` and have its documents ready.

```rust
let pdf = corvo.invoices().pdf("inv_…").await?;
std::fs::write(pdf.file_name.as_deref().unwrap_or("invoice.pdf"), &pdf.bytes)?;
```

### Verify webhook signatures

```rust
use corvo_nfse::verify_signature;

// raw_body: the request body bytes exactly as received (do not re-serialize);
// the header is X-Corvo-Signature.
if verify_signature(&raw_body, signature_header, "whsec_…") {
    // authentic: deduplicate by the event id, the same event can arrive twice
}
```

## Resources

| Namespace | Operations |
|---|---|
| `corvo.companies()` | `create`, `list`, `iterate`, `get`, `update`, `remove` |
| `corvo.certificates()` | `upload`, `list`, `remove` |
| `corvo.invoices()` | `create`, `calculate`, `list`, `iterate`, `get`, `events`, `pdf`, `xml`, `download_url`, `send_email` |
| `corvo.webhooks()` | `list`, `create`, `remove`, `test`, `.deliveries().{list, iterate, stats, retry}` |
| `corvo.credits()` | `balance`, `plans`, `history`, `iterate_history` |
| `corvo.billing()` | `plan`, `usage` |
| `corvo.plans()` | `list`, `get`, `quote` (public price catalog) |
| `corvo.health()` | `live`, `ready` |

Changing plans and the billing portal exist only in the dashboard, so they are not in the SDK.

## Configuration

| Option | Environment | Default |
|---|---|---|
| `api_key` | `CORVO_API_KEY` | required |
| `base_url` | `CORVO_BASE_URL` | `https://api.corvo.tech` |
| `prefix` | none | `v1` |
| `timeout` | none | 30s |
| `allow_insecure_http` | none | `false` (`http://` only for loopback and private hosts) |

## Behavior

- **Auth.** Every request sends `X-API-Key`. Keys are `corvo_test_...` or `corvo_live_...` and belong to one account and environment; the key decides the environment.
- **Responses.** Single resources and list items come back as the API's `data`; lists add `pagination`. Optional fields are `Option<_>`.
- **Errors.** `code` is the API's `ERROR_*` code, falling back to `http_<status>`; local failures use `network_error` and `timeout`. On `429`, `retry_after` holds the seconds to wait.

## Development

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test       # wiremock tests; the contract test is #[ignore] without env
cargo doc --no-deps
```

The contract test in `tests/contract.rs` runs only when `CORVO_API_KEY` and `CORVO_BASE_URL` point to a reachable instance.

## License

MIT, see [LICENSE](LICENSE).
