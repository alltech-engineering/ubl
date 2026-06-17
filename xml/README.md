# ubl-xml

UBL 2.5 XML serializer/deserializer. Manual `quick-xml` serialization with full
UBL namespace handling. Produces XSD-valid Invoice output.

This crate is the bridge between the XML-free `ubl-common`/`ubl-documents`
domain types and UBL 2.5 XML wire format.

## Directory Structure

```
src/
├── ser.rs      # Serializer: ToXml trait + Invoice/Document implementations
├── ns.rs       # Namespace constants + snake_case → PascalCase element naming
├── de.rs       # Deserializer: XML → domain types
├── error.rs    # Error types: Xml, MissingField, UnexpectedStructure, Io
└── lib.rs      # Public API: to_string(), from_str()
```

## Key Types

- **`ToXml` trait** — implement on any domain type to enable XML serialization
- **`to_string(doc, doc_type)`** — serialize any `ToXml` to an XML string
- **`from_str(xml)`** — deserialize XML back to domain types
- **`Error`** — unified error type covering XML, missing fields, structure errors, I/O

## Namespace Handling (`ns.rs`)

- `UBL` — document namespace: `urn:oasis:names:specification:ubl:schema:xsd:{Type}-2`
- `CAC` — Common Aggregate Components: `...CommonAggregateComponents-2`
- `CBC` — Common Basic Components: `...CommonBasicComponents-2`
- `to_element_name()` — converts Rust `snake_case` to UBL `PascalCase` with
  special handling for acronyms (`id` → `ID`, `uuid` → `UUID`, `uri` → `URI`)

## Usage

```rust
use ubl_xml::ser::ToXml;
use ubl_xml::ser::to_string;

let invoice: Invoice = ...;
let xml = to_string(&invoice, "Invoice")?;
// Produces:
// <?xml version="1.0" encoding="UTF-8"?>
// <Invoice xmlns="urn:oasis:names:specification:ubl:schema:xsd:Invoice-2"
//          xmlns:cac="..."
//          xmlns:cbc="...">
//   <cbc:ID>INV-001</cbc:ID>
//   <cbc:IssueDate>2026-06-14</cbc:IssueDate>
//   ...
// </Invoice>
```

## Design

- Uses `quick-xml` for high-performance XML I/O
- Domain crates stay XML-free — separation of concerns
- PascalCase element naming with acronym preservation
- Both serialization and deserialization paths
