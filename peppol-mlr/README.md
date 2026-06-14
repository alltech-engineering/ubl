# peppol-mlr

Peppol BIS Message Level Response 3.0 — Technical acknowledgment validation rules.

MLR is the simplest Peppol document. It serves as a transport/protocol-level
acknowledgment that a received message was syntactically valid and could be
parsed — it does not indicate business acceptance.

## Specification

- Peppol BIS MLR 3.0 — <https://docs.peppol.eu/poacc/upgrade-3/>
- Message Level Response transaction T71

## Rule Modules

| Module   | Description                         | Rules |
|----------|-------------------------------------|------:|
| `header` | Response code, document references, timestamps | 8 |

Total: 8 rules, 4 tests.

## Usage

```rust
use peppol_mlr::rules::mlr_rules;
use ubl_documents::status::ApplicationResponse;

let response: ApplicationResponse = /* parse or construct */;
let mut engine = mlr_rules(&response);
engine.evaluate_all();
```

## Dependencies

- `peppol-common` — shared RuleEngine and Rule types
- `ubl-documents` — UBL 2.1 ApplicationResponse struct
