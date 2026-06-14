# ubl-common

UBL 2.5 Common Basic Components (CBC) and Common Aggregate Components (CAC).

Pure Rust domain types for the OASIS UBL 2.5 standard. Every UBL element is a
Rust newtype for compiler-enforced type safety — you cannot accidentally assign
an ID to an Amount field.

## Directory Structure

```
src/
├── cbc/          # Common Basic Components (~1,335 type-safe newtypes)
│   ├── amount.rs     # Monetary amounts with currency (e.g., PayableAmount)
│   ├── code.rs       # Code values (e.g., DocumentTypeCode)
│   ├── identifier.rs # Identifiers (e.g., ID, UUID, GLN, GTIN)
│   ├── date.rs       # Dates (e.g., IssueDate, DueDate)
│   ├── text.rs       # Free-text fields (e.g., Note, Description)
│   ├── quantity.rs   # Quantities with unit codes
│   ├── measure.rs    # Physical measurements
│   ├── indicator.rs  # Boolean flags
│   ├── numeric.rs    # Numeric values (e.g., Rate, Percent)
│   └── binary.rs     # Binary object attachments
└── cac/          # Common Aggregate Components (57 domain aggregates)
    ├── party.rs, address.rs, contact.rs   # Party/address model
    ├── item.rs, price.rs                  # Item and pricing
    ├── line.rs, order_line.rs             # Document line types
    ├── tax.rs, allowance.rs               # Tax and allowances
    ├── delivery.rs, transport.rs          # Logistics
    ├── payment.rs, period.rs              # Payment terms
    ├── invoice_line.rs, credit_note_line  # Billing aggregates
    └── ... (document, signature, status, etc.)
```

## Key Concept: Type-Safe Newtypes

```rust
// Each UBL element is its own type — no confusion possible
pub struct ID(pub String);
pub struct IssueDate(pub NaiveDate);
pub struct PayableAmount { pub value: Decimal, pub currency_id: String }
```

## Usage

```rust
use ubl_common::cbc;
use ubl_common::cac;

let address = cac::PostalAddress {
    street_name: Some(cbc::StreetName("123 Main St".into())),
    city_name: Some(cbc::CityName("Cape Town".into())),
    country: Some(cac::Country {
        identification_code: Some(cbc::IdentificationCode("ZA".into())),
    }),
};
```

## Design Principles

- **XML-free**: No XML knowledge in the domain layer — serialization lives in `ubl-xml`
- **Serde-enabled**: All types derive `Serialize`/`Deserialize` for JSON round-tripping
- **No async**: Synchronous, lightweight, no runtime dependencies
- **Edition 2024**

## Dependencies

- `serde` (derive)
- `rust_decimal` (exact decimal arithmetic for amounts)
- `chrono` (date types)
