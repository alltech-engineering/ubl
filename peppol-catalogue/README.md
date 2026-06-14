# peppol-catalogue

Peppol BIS Catalogue 3.x — Product catalogue exchange validation rules.

## Specification

- Peppol BIS Catalogue 3.x — <https://docs.peppol.eu/poacc/upgrade-3/>
- Catalogue transaction T19

## Rule Modules

| Module   | Description                              | Rules |
|----------|------------------------------------------|------:|
| `header` | Catalogue identifiers, provider, validity |     8 |
| `lines`  | Catalogue line items, item properties    |     6 |

Total: 14 rules, 6 tests.

## Usage

```rust
use peppol_catalogue::rules::catalogue_rules;
use ubl_documents::catalogue::Catalogue;

let catalogue: Catalogue = /* parse or construct */;
let mut engine = catalogue_rules(&catalogue);
engine.evaluate_all();
```

## Dependencies

- `peppol-common` — shared RuleEngine and Rule types
- `ubl-documents` — UBL 2.1 Catalogue struct
