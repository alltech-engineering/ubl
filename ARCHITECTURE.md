# UBL 2.5 Rust Implementation — Architecture & Plan

Reference: https://docs.oasis-open.org/ubl/cs01-UBL-2.5/UBL-2.5.html

## 1. UBL 2.5 Overview

Universal Business Language (UBL) 2.5 defines a standard XML vocabulary for
electronic business documents. It covers the end-to-end supply chain, from
procurement through billing, transport, and regulatory reporting.

### Architecture Layers (from the spec)

```
Document Schemas (maindoc/)
    ├── Common Aggregate Components (CAC) — ~80 reusable ABIE structs
    ├── Common Basic Components (CBC) — ~200 reusable BBIE types
    └── Unqualified Data Types (UDT) — base XSD types
```

### Key Concepts

- **BBIE (Basic Business Information Entity):** A primitive field — Amount, Code,
  Date, Identifier, Name, Quantity, Text. Each has a specific UBL type (e.g.,
  `AmountType`, `CodeType`, `IdentifierType`).
- **ABIE (Aggregate Business Information Entity):** A complex reusable structure —
  Address, Party, Contact, Item, Price, TaxTotal, LineItem, etc.
- **Document Type:** A top-level business document assembled from BBIEs and ABIEs.

## 2. Complete Document Type Inventory (101 docs)

### Billing & Payment (10)
| # | Document | Use |
|---|----------|-----|
| 1 | Invoice | Request payment for goods/services |
| 2 | CreditNote | Reduce a previously issued invoice |
| 3 | DebitNote | Supplemental charge (revised role in 2.5) |
| 4 | SelfBilledInvoice | Invoice issued by buyer |
| 5 | SelfBilledCreditNote | Credit note issued by buyer |
| 6 | Reminder | Payment reminder for overdue invoice |
| 7 | Statement | Periodic account statement |
| 8 | UtilityStatement | Periodic utility billing statement |
| 9 | RemittanceAdvice | Notification of payment made |
| 10 | FreightInvoice | Invoice for freight services |

### Ordering (5)
| # | Document | Use |
|---|----------|-----|
| 11 | Order | Purchase order |
| 12 | OrderResponse | Full response to an order (line-level) |
| 13 | OrderResponseSimple | Simple accept/reject response |
| 14 | OrderChange | Modify an existing order |
| 15 | OrderCancellation | Cancel an existing order |

### Despatch & Fulfilment (6)
| # | Document | Use |
|---|----------|-----|
| 16 | DespatchAdvice | Notification of goods shipped |
| 17 | ReceiptAdvice | Confirmation of goods received |
| 18 | FulfilmentCancellation | Cancel a despatch/receipt advice |
| 19 | DeliveryNote | Accompanying delivery document |
| 20 | PackingList | Distribution of goods in packages |
| 21 | InstructionForReturns | Instructions for returning goods |

### Catalogue (5)
| # | Document | Use |
|---|----------|-----|
| 22 | Catalogue | Product/service listing |
| 23 | CatalogueRequest | Request for a catalogue |
| 24 | CatalogueItemSpecificationUpdate | Update item specs in catalogue |
| 25 | CataloguePricingUpdate | Update prices in catalogue |
| 26 | CatalogueDeletion | Delete items from catalogue |

### Quotation (2)
| # | Document | Use |
|---|----------|-----|
| 27 | RequestForQuotation | Request price quotation |
| 28 | Quotation | Price quotation response |

### Tendering & Pre-Award (22)
| # | Document | Use |
|---|----------|-----|
| 29 | Tender | Response to call for tenders |
| 30 | TenderReceipt | Acknowledgement of tender receipt |
| 31 | TenderStatus | Inquiry/notification about tender status |
| 32 | TenderStatusRequest | Request for tender status |
| 33 | TenderWithdrawal | Withdraw a submitted tender |
| 34 | TendererQualification | Qualification submission |
| 35 | TendererQualificationResponse | Response to qualification |
| 36 | TenderContract | Contract resulting from tender |
| 37 | AwardedNotification | Notification of contract award |
| 38 | UnawardedNotification | Notification of non-award |
| 39 | CallForTenders | Invitation to submit tenders |
| 40 | ContractNotice | Notice of a awarded contract |
| 41 | ContractAwardNotice | Notice of contract award |
| 42 | PriorInformationNotice | Advance notice of procurement |
| 43 | ExpressionOfInterestRequest | Request to express interest |
| 44 | ExpressionOfInterestResponse | Response to expression of interest |
| 45 | QualificationApplicationRequest | Request for qualification |
| 46 | QualificationApplicationResponse | Qualification response |
| 47 | UnsubscribeFromProcedureRequest | Withdraw from procedure |
| 48 | UnsubscribeFromProcedureResponse | Confirmation of withdrawal |
| 49 | Enquiry | Pre-award question from supplier |
| 50 | EnquiryResponse | Response to enquiry |

### Transportation & Logistics (16)
| # | Document | Use |
|---|----------|-----|
| 51 | BillOfLading | Contract of carriage by sea |
| 52 | Waybill | Non-negotiable transport document |
| 53 | CertificateOfOrigin | Declaration of goods origin |
| 54 | ForwardingInstructions | Instructions to freight forwarder |
| 55 | TransportationStatus | Status of transport |
| 56 | TransportationStatusRequest | Request for transport status |
| 57 | TransportExecutionPlan | Agreed transport plan |
| 58 | TransportExecutionPlanRequest | Request transport plan |
| 59 | TransportServiceDescription | Available transport services |
| 60 | TransportServiceDescriptionRequest | Request service description |
| 61 | TransportProgressStatus | Transport progress report |
| 62 | TransportProgressStatusRequest | Request progress status |
| 63 | GoodsItemItinerary | Route of goods items |
| 64 | GoodsItemPassport | Temporary export/import passport |
| 65 | Manifest | Summary of cargo on transport means |
| 66 | CommonTransportationReport | Transport report to authorities |

### Inventory & Forecasting (7)
| # | Document | Use |
|---|----------|-----|
| 67 | InventoryReport | Current inventory levels |
| 68 | StockAvailabilityReport | Available stock report |
| 69 | ProductActivity | Product movement/activity |
| 70 | ItemInformationRequest | Request item master data |
| 71 | Forecast | Demand/supply forecast |
| 72 | ForecastRevision | Revised forecast |
| 73 | TradeItemLocationProfile | Item location data |

### Status & Inquiries (9)
| # | Document | Use |
|---|----------|-----|
| 74 | ApplicationResponse | Response to any business document |
| 75 | DocumentStatus | Status of a previously sent document |
| 76 | DocumentStatusRequest | Request document status |
| 77 | InvoiceStatusRequest | Request invoice processing status |
| 78 | InvoiceStatusResponse | Response about invoice status |
| 79 | ExceptionCriteria | Criteria for exception reporting |
| 80 | ExceptionNotification | Notification of exception |
| 81 | ProcurementStatus | Status of procurement process |
| 82 | ProcurementStatusRequest | Request procurement status |

### Customs & Regulatory (9)
| # | Document | Use |
|---|----------|-----|
| 83 | ExportCustomsDeclaration | Export customs declaration |
| 84 | ImportCustomsDeclaration | Import customs declaration |
| 85 | TransitCustomsDeclaration | Transit customs declaration |
| 86 | ProofOfReexportation | Proof of re-export |
| 87 | ProofOfReexportationReminder | Reminder for proof |
| 88 | ProofOfReexportationRequest | Request for proof |
| 89 | GoodsCertificate | Certificate for goods |
| 90 | GuaranteeCertificate | Financial guarantee certificate |
| 91 | PurchaseReceipt | Receipt for purchase |

### Business Directory & Agreements (5)
| # | Document | Use |
|---|----------|-----|
| 92 | BusinessCard | Trading capability info |
| 93 | BusinessInformation | Business registration/notification |
| 94 | DigitalAgreement | Trading partner agreement |
| 95 | DigitalCapability | Digital trading capability |
| 96 | AttachedDocument | (Deprecated in 2.5 — use XHE) |

### Waste Management (2)
| # | Document | Use |
|---|----------|-----|
| 97 | WasteMovement | Waste consignment movement |
| 98 | WasteNotification | Waste shipment notification |

### Other (3)
| # | Document | Use |
|---|----------|-----|
| 99 | RetailEvent | Retail event information |
| 100 | WeightStatement | Weight certificate |
| 101 | WorkReport | Work completion report |

## 3. Common Components

### Common Basic Components (CBC) — ~200 types

Primitive value types, each with unique semantics:
- **Amount:** `AmountType` — monetary value with currency
- **Code:** ~50 code types (CurrencyCode, CountryCode, UnitCode, etc.)
- **Date/Time:** `DateType`, `TimeType`, `DateTimeType`
- **Identifier:** ~40 ID types (UUID, GLN, GTIN, etc.)
- **Name:** `NameType` — business names
- **Numeric:** `NumericType`, `PercentType`, `RateType`, `ValueType`
- **Quantity:** `QuantityType` — measured quantities with unit
- **Text:** `TextType` — free-form text with optional language
- **Indicator:** `IndicatorType` — boolean flags
- **Measure:** `MeasureType` — physical measurements
- **Binary:** `BinaryObjectType` — embedded files (PDFs, images)

Each CBC type in the XSD is a named element (e.g., `cbc:InvoiceTypeCode`,
`cbc:TaxAmount`, `cbc:IssueDate`). In Rust, these become newtypes wrapping a
base type with an associated const name.

### Common Aggregate Components (CAC) — ~80 types

Reusable complex structures. Key aggregates:

| Aggregate | Purpose |
|-----------|---------|
| Address | Physical/postal address |
| AddressLine | Single address line |
| AllowanceCharge | Discount or surcharge |
| Attachment | Referenced or embedded document |
| BillingReference | Reference to an invoice |
| Branch | Division of an organization |
| CatalogueLine | Line in a catalogue |
| ClassificationScheme | Classification system |
| CommodityClassification | Item classification |
| Communication | Phone, email, web |
| Contact | Person contact details |
| Contract | Contractual agreement info |
| Country | Country identification |
| CustomerParty | Customer role |
| Delivery | Delivery information |
| DeliveryTerms | Delivery terms (Incoterms) |
| Dimension | Physical dimensions |
| DocumentReference | Reference to another document |
| ExternalReference | URI or external reference |
| FinancialAccount | Bank account details |
| Item | Item of sale/trade |
| ItemIdentification | Item identifier (GTIN, etc.) |
| ItemInstance | Specific tracked instance |
| ItemProperty | Item characteristic |
| Language | Language specification |
| LegalTotal | Invoice legal monetary total |
| LineItem | Line in order/invoice |
| Location | Geographic location |
| LotIdentification | Batch/lot identifier |
| MonetaryTotal | Total amounts summary |
| OrderLine | Order line item |
| OrderReference | Reference to an order |
| Party | Organization or individual |
| PartyIdentification | Party identifier |
| PartyLegalEntity | Legal registration info |
| PartyName | Name of a party |
| PartyTaxScheme | Tax registration |
| Payment | Payment information |
| PaymentMeans | How payment is made |
| PaymentTerms | When payment is due |
| Period | Date range |
| Person | Individual person |
| PostalAddress | Mailing address |
| Price | Price with amount and quantity |
| PricingReference | Reference to pricing info |
| SupplierParty | Supplier role |
| TaxCategory | Tax rate and category |
| TaxScheme | Tax authority/regime |
| TaxSubtotal | Tax breakdown |
| TaxTotal | Total tax amount |
| Temperature | Temperature conditions |
| TransportEquipment | Container/trailer |
| TransportHandlingUnit | Pallet/package |
| TransportMeans | Vehicle/vessel |
| TransportationService | Transport service details |

## 4. Rust Architecture Design

### Crate Structure

```
ubl/
├── Cargo.toml                  # Workspace root
├── ARCHITECTURE.md             # This file
├── ubl-common/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── cbc/                # Common Basic Components
│       │   ├── mod.rs
│       │   ├── amount.rs       # AmountType
│       │   ├── code.rs         # All code types
│       │   ├── date.rs         # DateType, TimeType, DateTimeType
│       │   ├── identifier.rs   # All identifier types
│       │   ├── name.rs         # NameType
│       │   ├── numeric.rs      # NumericType, RateType, ValueType
│       │   ├── quantity.rs     # QuantityType
│       │   ├── text.rs         # TextType
│       │   ├── indicator.rs    # IndicatorType
│       │   ├── measure.rs      # MeasureType
│       │   └── binary.rs       # BinaryObjectType
│       └── cac/                # Common Aggregate Components
│           ├── mod.rs
│           ├── address.rs
│           ├── party.rs
│           ├── contact.rs
│           ├── item.rs
│           ├── price.rs
│           ├── tax.rs
│           ├── payment.rs
│           ├── delivery.rs
│           ├── document.rs
│           ├── transport.rs
│           └── ...80 modules total
└── ubl-documents/
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── billing/            # Invoice, CreditNote, etc.
        ├── ordering/           # Order, OrderResponse, etc.
        ├── despatch/           # DespatchAdvice, ReceiptAdvice, etc.
        ├── catalogue/          # Catalogue, CatalogueRequest, etc.
        ├── quotation/          # RequestForQuotation, Quotation
        ├── tendering/          # Tender, CallForTenders, etc.
        ├── transportation/     # BillOfLading, Waybill, etc.
        ├── inventory/          # InventoryReport, etc.
        ├── status/             # ApplicationResponse, DocumentStatus, etc.
        ├── customs/            # ExportCustomsDeclaration, etc.
        ├── directory/          # BusinessCard, DigitalAgreement, etc.
        ├── waste/              # WasteMovement, WasteNotification
        └── other/              # RetailEvent, WeightStatement, WorkReport
```

### Design Principles

1. **Newtype wrappers for all CBC types.** `InvoiceTypeCode(String)` not
   `type InvoiceTypeCode = String;`. This prevents cross-type confusion.

2. **Value objects with validation.** Each type validates on construction:
   ```rust
   pub struct Amount { value: Decimal, currency: CurrencyCode }
   pub struct Quantity { value: Decimal, unit: UnitCode }
   ```

3. **No XML dependency in the domain layer.** The domain types are pure Rust
   structs. XML (de)serialization lives in adapters (future crate).

4. **serde for JSON representation.** All types derive Serialize/Deserialize.
   XML mapping is a separate concern.

5. **Composition over inheritance.** Document types compose CAC aggregates:
   ```rust
   pub struct Invoice {
       pub id: Identifier,
       pub issue_date: Date,
       pub invoice_type_code: Code,
       pub accounting_supplier_party: SupplierParty,   // CAC
       pub accounting_customer_party: CustomerParty,   // CAC
       pub tax_total: Vec<TaxTotal>,                   // CAC
       pub legal_monetary_total: LegalTotal,           // CAC
       pub invoice_line: Vec<InvoiceLine>,             // CAC
   }
   ```

6. **Optionality matches UBL cardinality.** Fields use `Option<T>` for 0..1,
   `Vec<T>` for 0..n, and bare `T` for 1..1 required fields.

7. **Module naming follows UBL terminology.** Use the exact UBL names
   (DespatchAdvice not DeliveryNote, FulfilmentCancellation with UK spelling).

### Deprecation Handling

UBL 2.5 marks certain components as deprecated (to be removed in a future
major version). In Rust:
- Deprecated document types get `#[deprecated]` attribute
- Deprecated fields get `#[deprecated]` with notes pointing to replacements
- The crate compiles clean — deprecation is advisory, not breaking

## 5. Implementation Plan — Phases

### Phase 0: Foundation (ubl-common)
- [ ] **CBC Primitives:** All ~200 basic types as Rust newtypes
  - Amount types, code types, date/time types, identifier types
  - Name, text, numeric, quantity, indicator, measure, binary
- [ ] **CBC Tests:** Construction, validation, equality
- [ ] **CAC Aggregates — Tier 1 (core business):** Party, Address, Contact,
  Item, Price, TaxTotal, LegalTotal, MonetaryTotal, PaymentMeans,
  PaymentTerms, DeliveryTerms, AllowanceCharge
- [ ] **CAC Aggregates — Tier 2 (document support):** DocumentReference,
  BillingReference, OrderReference, LineItem, InvoiceLine, OrderLine
- [ ] **CAC Aggregates — Tier 3 (extended):** Transport, Delivery,
  Location, Period, Dimension, Temperature
- [ ] **CAC Aggregates — Tier 4 (remaining):** All other aggregates

### Phase 1: Core Billing (ubl-documents)
- [ ] Invoice (highest priority — most used UBL document)
- [ ] CreditNote
- [ ] SelfBilledInvoice
- [ ] SelfBilledCreditNote

### Phase 2: Billing Extended
- [ ] DebitNote
- [ ] Reminder
- [ ] Statement
- [ ] UtilityStatement
- [ ] RemittanceAdvice
- [ ] FreightInvoice

### Phase 3: Ordering
- [ ] Order
- [ ] OrderResponse
- [ ] OrderResponseSimple
- [ ] OrderChange
- [ ] OrderCancellation

### Phase 4: Despatch & Fulfilment
- [ ] DespatchAdvice
- [ ] ReceiptAdvice
- [ ] FulfilmentCancellation
- [ ] DeliveryNote
- [ ] PackingList
- [ ] InstructionForReturns

### Phase 5: Catalogue
- [ ] Catalogue
- [ ] CatalogueRequest
- [ ] CatalogueItemSpecificationUpdate
- [ ] CataloguePricingUpdate
- [ ] CatalogueDeletion

### Phase 6: Quotation & Tendering
- [ ] RequestForQuotation, Quotation
- [ ] All 20 tendering documents

### Phase 7: Transportation & Logistics
- [ ] All 16 transport documents

### Phase 8: Remaining Documents
- [ ] Inventory & Forecasting (7)
- [ ] Status & Inquiries (9)
- [ ] Customs & Regulatory (9)
- [ ] Directory & Agreements (5)
- [ ] Waste (2)
- [ ] Other (3)

## 6. Key Design Decisions

1. **Two-crate vs multi-crate.** Chosen: two crates (ubl-common + ubl-documents).
   Splitting into per-domain crates (ubl-billing, ubl-ordering, etc.) adds
   compile-time overhead without meaningful isolation benefit at this scale.
   Reevaluate if compile times become painful.

2. **Newtype vs type alias for CBC.** Chosen: newtype. `InvoiceTypeCode` and
   `CreditNoteTypeCode` are both strings but must not be confused. The compiler
   should enforce this.

3. **serde for JSON, not XML.** Chosen: serde with JSON as the primary wire
   format for the domain layer. XML adapters will be a separate crate under
   a future `ubl-xml` or adapter. The domain must not know about XML.

4. **No async in domain.** The domain crates have zero dependencies on tokio,
   axum, or any async runtime. Pure sync code with validation.

5. **Edition 2024.** All crates use Rust edition 2024 for latest language
   features. Minimum supported Rust version: 1.85+.
