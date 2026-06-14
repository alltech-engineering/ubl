# peppol-xml

Peppol-aware XML serialization. Wraps `ubl-xml` with Peppol BIS identifiers —
the `CustomizationID` and `ProfileID` elements that declare a UBL document as
Peppol-compliant.

## Directory Structure

```
src/
└── lib.rs   # PeppolDocument<T> wrapper + to_peppol_xml() function + helpers
```

## Key Types

- **`PeppolDocument<T: ToXml>`** — generic wrapper that pairs any UBL document
  with its Peppol identity and root element metadata

  ```rust
  pub struct PeppolDocument<T: ToXml> {
      pub document: T,               // The UBL document (Invoice, Order, etc.)
      pub identity: DocumentIdentity, // CustomizationID + ProfileID
      pub root_element: &'static str, // e.g., "Invoice"
      pub root_namespace: &'static str,// e.g., "urn:oasis:...Invoice-2"
  }
  ```

- **`to_peppol_xml(doc)`** — serialize a `PeppolDocument<T>` to a full Peppol
  XML string with namespace declarations, XML declaration, and identity elements

- **`write_cbc_with_scheme()`** — helper for writing CBC elements with
  `schemeID` attributes (used for `EndpointID`, etc.)

## Usage

```rust
use ubl_documents::billing::Invoice;
use peppol_common::identity::identities;
use peppol_xml::{PeppolDocument, to_peppol_xml};

// 1. Build a UBL Invoice (from JSON, programmatically, etc.)
let invoice: Invoice = serde_json::from_str(json_str)?;

// 2. Wrap it with Peppol metadata
let peppol = PeppolDocument {
    document: invoice,
    identity: identities::billing_3_0("Invoice"),
    root_element: "Invoice",
    root_namespace: "urn:oasis:names:specification:ubl:schema:xsd:Invoice-2",
};

// 3. Serialize to Peppol XML
let xml = to_peppol_xml(&peppol)?;
// Output includes:
//   <cbc:CustomizationID>urn:fdc:peppol.eu:2017:poacc:billing:3.0::2.1</cbc:CustomizationID>
//   <cbc:ProfileID>urn:fdc:peppol.eu:2017:poacc:billing:01:1.0</cbc:ProfileID>
```

## Output Structure

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Invoice xmlns="urn:oasis:names:specification:ubl:schema:xsd:Invoice-2"
         xmlns:cac="urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"
         xmlns:cbc="urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2">
  <cbc:CustomizationID>urn:fdc:peppol.eu:2017:poacc:billing:3.0::2.1</cbc:CustomizationID>
  <cbc:ProfileID>urn:fdc:peppol.eu:2017:poacc:billing:01:1.0</cbc:ProfileID>
  <!-- ... rest of UBL document from ToXml impl ... -->
</Invoice>
```

## Design

- Thin wrapper over `ubl-xml` — delegates all domain serialization to the
  `ToXml` trait
- Handles Peppol namespace prefixes and identity elements
- Zero-copy design: `PeppolDocument<T>` borrows its contents
