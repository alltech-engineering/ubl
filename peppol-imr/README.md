# peppol-imr

Peppol BIS Invoice Message Response 3.0 — Business-level invoice approval and
rejection validation rules.

IMR conveys the business decision on a received invoice: approve, reject, or
flag as disputed. It uses the same `ApplicationResponse` struct as MLR but
applies different business rules (acceptance/rejection codes, line-level
status, reason codes).

## Specification

- Peppol BIS IMR 3.0 — <https://docs.peppol.eu/poacc/upgrade-3/>
- Invoice Response transaction T111

## Rule Modules

| Module   | Description                                       | Rules |
|----------|---------------------------------------------------|------:|
| `header` | Response status, invoice reference, reason codes  |     7 |

Total: 7 rules, 4 tests.

## Usage

```rust
use peppol_imr::rules::imr_rules;
use ubl_documents::status::ApplicationResponse;

let response: ApplicationResponse = /* parse or construct */;
let mut engine = imr_rules(&response);
engine.evaluate_all();
```

## Dependencies

- `peppol-common` — shared RuleEngine and Rule types
- `ubl-documents` — UBL 2.1 ApplicationResponse struct
