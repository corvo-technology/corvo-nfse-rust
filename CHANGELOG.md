# Changelog

## 0.3.0

Aligns the SDK with API contract 0.3.0 (94 operations).

### Breaking

- Plan slugs are now `free`, `solo`, `equipe`, `empresa`, `escala` (and `self-hosted`); `developer`, `growth`, `scale` and `volume` are gone. Slugs are open strings.
- `invoices().create` returns `InvoiceCreated` (an invoice, or a `Schedule` for `SCHEDULE` and `RECURRING`); use `.into_invoice()` / `.into_schedule()`.
- `CreditBalance`: removed `tenant_id`, `overage_units` and `overage_amount_cents`; added `plan_remaining` and `avulso_balance`; `balance` and `hard_limit` are `i64` (`balance` includes the loose notes).
- `BillingUsage`: removed `overage_units` and `overage_amount_cents`; added `avulso_balance`; `hard_limit` is `i64`. `BillingSubscription` gained `stripe_portal_available`, `stripe_subscription` and `pix_subscription`. The 404 of a workspace without a plan has code `NO_PLAN_ASSIGNED`.
- `Invoice` follows the contract: removed `document_key_version`; `service_snapshot.{code, description, municipality_code}`, `fiscal_provider` and `fiscal_schema_version` are required; `documents_status` and the e-mail statuses are enums; `InvoiceStage` has `Cancelled`.
- `InvoiceCustomerInput.tax_id` is now `Option<String>` (a customer outside Brazil uses `foreign`).
- `invoices().send_email` takes the optional destination: `send_email(id, email, idempotency_key)`.
- `InvoiceEvent.from_status` / `to_status` are strings.
- `Company` no longer has `deleted_at`; `TaxRegime` has `LucroPresumido` and `LucroReal`.
- `WebhookDeliveryListParams.hours` and `deliveries().stats(hours)` are `f64`. `WebhookTestResult` carries `error` and `status_code`.
- `Webhook` has `environment`: a test key can no longer reach live webhooks; URLs on private networks are rejected. `WEBHOOK_EVENTS` has 12 events (`nfse.invoice.taker_rejected`).

### Added

- Public plan prices, no API key needed: `plans().list(limit, cursor, cycle)` and `plans().get(slug, cycle)` return `PlanPrice` (`monthly_equivalent_cents`, `charge_total_cents`; `cycle` is `monthly` or `annual`). `PlanQuote` is replaced by `PlanPrice`.

- Resources: `customers()`, `cnpj()`, `service_favorites()`, `accountant_package()`, `invoice_package()`, `schedules()`, `imports()` (multipart upload, template and errors CSV), `fiscal_profiles()`, `service_tax_profiles()`, `dps_sequence()`, `integrations()`, `integration_connections()`, `integration_operations()`, `fiscal_portal()`.
- `invoices()`: `substitute`, `overview`, `review`; `customer_tax_id` filter on `list`; `purchase_order`, `notes`, `options.recurrence` and `customer.foreign` on create; new invoice fields `substitutes_invoice_id`, `substitution_description`, `portal_flag`.
- `credits().preview_purchase(units)` (`GET /credits/purchase/preview`).
- `tests/contract_coverage.rs` checks every operationId of the vendored `spec/openapi.json`.

### Changed

- `invoice_package().summary` returns `total` as a decimal `String` (`"1234.50"`), not a number.
- `CompanyUpdateParams.tax_id` is removed: the CNPJ cannot be changed.
- `invoices().send_email` without an address and `imports().emit` without `send_email` send no body.
- Nullable fields of the contract (company address, trade name, rates; invoice e-mail status, cancellation reason, portal flag; calculation `classification`, `applicability`, `calculation`) deserialize as `None`/null. `error.details` of a batch result can be a list or an object.

- Version, `User-Agent` and `VERSION` are `0.3.0`.
- `Transport` supports `PUT`; downloads can set `Accept` (ZIP, CSV).
- `POST /invoices/calculate` is read as `{ data }` (the API wraps it, although `openapi.json` shows it bare).
