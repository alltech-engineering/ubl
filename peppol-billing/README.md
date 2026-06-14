# peppol-billing

Peppol BIS Billing 3.0 — Invoice and CreditNote validation rules.

## Specification

- EN 16931 (European Norm for e-Invoicing)
- Peppol BIS Billing 3.0 — <https://docs.peppol.eu/poacc/billing/3.0/>

## Rule Modules

| Module          | Description                             | Rules |
|-----------------|-----------------------------------------|------:|
| `header`        | Document-level identifiers and metadata |     9 |
| `parties`       | Buyer, Seller, Tax Representative, etc. |    10 |
| `lines`         | Invoice line items                      |     5 |
| `tax_calc`      | Tax totals, breakdowns, VAT categories  |     7 |
| `code_lists`    | Peppol and UN/CEFACT code list checks   |     7 |
| `constraints`   | Cross-field and co-occurrence rules     |    37 |
| `national`      | Country-specific national rules         |    64 |
| `za`            | South Africa (ZA) specific rules        |     4 |
| `credit_note`   | CreditNote document rules               |    12 |

Total: 155 rules, 31 tests.

## Usage

```rust
use peppol_billing::rules::{billing_rules, credit_note_rules};
use ubl_documents::billing::{Invoice, CreditNote};

// Validate an Invoice
let invoice: Invoice = /* parse or construct */;
let mut engine = billing_rules(&invoice);
engine.evaluate_all();
for result in engine.results() {
    println!("{}: {:?}", result.rule_id, result.outcome);
}

// Validate a CreditNote
let credit_note: CreditNote = /* parse or construct */;
let mut engine = credit_note_rules(&credit_note);
engine.evaluate_all();
```

## Dependencies

- `peppol-common` — shared RuleEngine and Rule types
- `ubl-documents` — UBL 2.1 Invoice and CreditNote structs
