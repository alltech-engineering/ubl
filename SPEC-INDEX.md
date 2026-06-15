# UBL 2.5 Type Location Index

Where to find each UBL type definition in the XSD files.

## Key: understanding XSD locations

- `CBC` = Common Basic Components (`ubl-common/spec/common/UBL-CommonBasicComponents-2.5.xsd`)
- `CAC` = Common Aggregate Components (`ubl-common/spec/common/UBL-CommonAggregateComponents-2.5.xsd`)
- `MAINDOC` = Document schemas (`ubl-documents/spec/maindoc/UBL-{DocName}-2.5.xsd`)

## Core types used by Peppol crates

| Rust Struct | UBL Type | XSD File | Line |
|---|---|---|---|
| Invoice | Invoice | MAINDOC UBL-Invoice-2.5.xsd | — |
| CreditNote | CreditNote | MAINDOC UBL-CreditNote-2.5.xsd | — |
| Order | Order | MAINDOC UBL-Order-2.5.xsd | — |
| OrderResponse | OrderResponse | MAINDOC UBL-OrderResponse-2.5.xsd | — |
| DespatchAdvice | DespatchAdvice | MAINDOC UBL-DespatchAdvice-2.5.xsd | — |
| ApplicationResponse | ApplicationResponse | MAINDOC UBL-ApplicationResponse-2.5.xsd | — |
| Catalogue | Catalogue | MAINDOC UBL-Catalogue-2.5.xsd | — |
| OrderLine | OrderLineType | CAC | 29726 |
| InvoiceLine | InvoiceLineType | CAC | 15040 |
| DespatchLine | DespatchLineType | CAC | 8092 |
| LineItem | LineItemType | CAC | 15772 |
| Party | PartyType | CAC | 23349 |
| PostalAddress | AddressType | CAC | 855 |
| Contact | ContactType | CAC | 4259 |
| Item | ItemType | CAC | 13749 |
| Price | PriceType | CAC | 24549 |
| TaxTotal | TaxTotalType | CAC | 30306 |
| TaxSubtotal | TaxSubtotalType | CAC | 30226 |
| TaxCategory | TaxCategoryType | CAC | 30046 |
| LegalMonetaryTotal | MonetaryTotalType | CAC | 19119 |
| Delivery | DeliveryType | CAC | 6466 |
| Shipment | ShipmentType | CAC | 26204 |
| PaymentMeans | PaymentMeansType | CAC | 24367 |
| PaymentTerms | PaymentTermsType | CAC | 24404 |
| AllowanceCharge | AllowanceChargeType | CAC | 960 |
| DocumentReference | DocumentReferenceType | CAC | 7435 |
| Period | PeriodType | CAC | 24229 |
| SupplierParty | SupplierPartyType | CAC | 28947 |
| CustomerParty | CustomerPartyType | CAC | 5856 |

## Finding any type

All 1,202 CBC types are in `UBL-CommonBasicComponents-2.5.xsd`.
All 80 CAC types are in `UBL-CommonAggregateComponents-2.5.xsd`.
All 101 document types are in `ubl-documents/spec/maindoc/`.

To find an aggregate type: `grep "complexType name=\"TypeName\"" ubl-common/spec/common/UBL-CommonAggregateComponents-2.5.xsd`
To find a basic type: `grep "element name=\"TypeName\"" ubl-common/spec/common/UBL-CommonBasicComponents-2.5.xsd`
