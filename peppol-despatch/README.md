# peppol-despatch

Peppol BIS Despatch Advice 3.x — Shipment and delivery validation rules.

## Specification

- Peppol BIS Despatch Advice 3.x — <https://docs.peppol.eu/poacc/upgrade-3/>
- Despatch Advice transaction T16

## Rule Modules

| Module      | Description                          | Rules |
|-------------|--------------------------------------|------:|
| `header`    | Document identifiers, dates, order refs | 12 |
| `parties`   | Shipper, Carrier, Delivery parties   |     9 |
| `lines`     | Despatch line items and quantities   |     9 |
| `shipment`  | Shipment-level details and tracking  |    12 |

Total: 42 rules, 10 tests.

## Usage

```rust
use peppol_despatch::rules::despatch_rules;
use ubl_documents::despatch::DespatchAdvice;

let despatch: DespatchAdvice = /* parse or construct */;
let mut engine = despatch_rules(&despatch);
engine.evaluate_all();
```

## Dependencies

- `peppol-common` — shared RuleEngine and Rule types
- `ubl-documents` — UBL 2.1 DespatchAdvice struct
