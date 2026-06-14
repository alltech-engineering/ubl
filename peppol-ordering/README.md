# peppol-ordering

Peppol BIS Ordering 3.x — Purchase Order and OrderResponse validation rules.

## Specification

- Peppol BIS Ordering 3.x — <https://docs.peppol.eu/poacc/upgrade-3/>
- Order transaction T01 (Purchase Order)
- Order Response transaction T76

## Rule Modules

| Module            | Description                             | Rules |
|-------------------|-----------------------------------------|------:|
| `header`          | Document identifiers, dates, references |    13 |
| `parties`         | Buyer, Seller, and other parties        |    12 |
| `lines`           | Order line items and quantities         |     8 |
| `delivery`        | Delivery terms and locations            |    10 |
| `payment`         | Payment means and terms                 |     9 |
| `code_lists`      | Peppol code list validations            |     8 |
| `order_response`  | OrderResponse-specific rules            |    18 |

Total: 78 rules, 17 tests.

## Usage

```rust
use peppol_ordering::rules::{ordering_rules, ordering_response_rules};
use ubl_documents::ordering::{Order, OrderResponse};

// Validate a Purchase Order
let order: Order = /* parse or construct */;
let mut engine = ordering_rules(&order);
engine.evaluate_all();

// Validate an Order Response
let response: OrderResponse = /* parse or construct */;
let mut engine = ordering_response_rules(&response);
engine.evaluate_all();
```

## Dependencies

- `peppol-common` — shared RuleEngine and Rule types
- `ubl-documents` — UBL 2.1 Order and OrderResponse structs
