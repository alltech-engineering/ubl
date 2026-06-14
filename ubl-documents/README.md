# ubl-documents

UBL 2.5 Business Document Types — 101 complete document definitions built on
top of `ubl-common`.

Each document type is a Rust struct that composes CBC and CAC types into a
full schema-compliant document. All types are `Serialize`/`Deserialize` for
JSON round-tripping.

## Directory Structure

```
src/
├── billing/          # Core billing documents
│   ├── invoice.rs             # Invoice
│   ├── credit_note.rs         # CreditNote
│   ├── debit_note.rs          # DebitNote
│   ├── self_billed_invoice.rs # SelfBilledInvoice
│   ├── self_billed_credit_note.rs
│   ├── statement.rs           # Statement
│   ├── remittance_advice.rs   # RemittanceAdvice
│   ├── reminder.rs            # Reminder
│   └── ...
├── ordering/         # Purchase order lifecycle
│   ├── order.rs               # Order
│   ├── order_response.rs      # OrderResponse
│   ├── order_change.rs        # OrderChange
│   └── order_cancellation.rs  # OrderCancellation
├── despatch/         # Shipping/despatch documents
│   ├── despatch_advice.rs     # DespatchAdvice
│   └── receipt_advice.rs      # ReceiptAdvice
├── catalogue.rs      # Catalogue
├── quotation.rs      # Quotation
├── tendering.rs      # Tender documents
├── transportation/   # Transport documents
├── customs/          # Customs documents
├── directory/        # Directory/business card documents
├── inventory/        # Inventory reports and forecasts
├── status/           # Document status tracking
├── waste/            # Waste management documents
└── other/            # Miscellaneous documents
```

## Key Types

| Module | Key Structs |
|--------|-------------|
| `billing` | `Invoice`, `CreditNote`, `DebitNote`, `Statement`, `RemittanceAdvice` |
| `ordering` | `Order`, `OrderResponse`, `OrderChange`, `OrderCancellation` |
| `despatch` | `DespatchAdvice`, `ReceiptAdvice` |
| `catalogue` | `Catalogue` |
| `quotation` | `Quotation` |

## Usage

```rust
use ubl_documents::billing::Invoice;
use ubl_common::cbc;

let invoice = Invoice {
    id: Some(cbc::ID("INV-001".into())),
    issue_date: Some(cbc::IssueDate(NaiveDate::from_ymd_opt(2026, 6, 14).unwrap())),
    document_currency_code: Some(cbc::DocumentCurrencyCode("ZAR".into())),
    // ... supplier party, lines, totals
};

// Serialize to/from JSON
let json = serde_json::to_string_pretty(&invoice)?;
let parsed: Invoice = serde_json::from_str(&json)?;
```

## Design

- No XML knowledge — delegates serialization to `ubl-xml`
- All 101 UBL 2.5 document types
- Serde-enabled for JSON round-tripping
- Depends only on `ubl-common` + `serde`
