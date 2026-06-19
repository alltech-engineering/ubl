# EXT — UBL 2.5 Extension Components

Generated from OASIS UBL 2.5 XSD annotations. 9 types.

## Types

- [UBLExtensions](#ublextensions) — A container for all extensions present in the document.
- [UBLExtension](#ublextension) — A single extension for private use.
- [ExtensionAgencyID](#extensionagencyid) — 
- [ExtensionAgencyName](#extensionagencyname) — 
- [ExtensionAgencyURI](#extensionagencyuri) — 
- [ExtensionReason](#extensionreason) — 
- [ExtensionReasonCode](#extensionreasoncode) — 
- [ExtensionURI](#extensionuri) — 
- [ExtensionVersionID](#extensionversionid) — 

### UBLExtensions

**XSD type:** `UBLExtensionsType`
**Definition:** A container for all extensions present in the document.

**Fields:**

| Field | Type | Card. | Definition |
|-------|------|-------|------------|
| `UBLExtension` | `UBLExtension` | 1..n | A single extension for private use. |

### UBLExtension

**XSD type:** `UBLExtensionType`
**Definition:** A single extension for private use.

**Fields:**

| Field | Type | Card. | Definition |
|-------|------|-------|------------|
| `ID` | `cbc:ID` | 0..1 | An identifier for the Extension assigned by the creator of the extension. |
| `Name` | `cbc:Name` | 0..1 | A name for the Extension assigned by the creator of the extension. |
| `ExtensionAgencyID` | `ExtensionAgencyID` | 0..1 | An agency that maintains one or more Extensions. |
| `ExtensionAgencyName` | `ExtensionAgencyName` | 0..1 | The name of the agency that maintains the Extension. |
| `ExtensionVersionID` | `ExtensionVersionID` | 0..1 | The version of the Extension. |
| `ExtensionAgencyURI` | `ExtensionAgencyURI` | 0..1 | A URI for the Agency that maintains the Extension. |
| `ExtensionURI` | `ExtensionURI` | 0..1 | A URI for the Extension. |
| `ExtensionReasonCode` | `ExtensionReasonCode` | 0..1 | A code for reason the Extension is being included. |
| `ExtensionReason` | `ExtensionReason` | 0..1 | A description of the reason for the Extension. |
| `ExtensionContent` | `ExtensionContent` | 1..1 | The definition of the extension content. |

### ExtensionAgencyID

**XSD type:** `ExtensionAgencyIDType`

### ExtensionAgencyName

**XSD type:** `ExtensionAgencyNameType`

### ExtensionAgencyURI

**XSD type:** `ExtensionAgencyURIType`

### ExtensionReason

**XSD type:** `ExtensionReasonType`

### ExtensionReasonCode

**XSD type:** `ExtensionReasonCodeType`

### ExtensionURI

**XSD type:** `ExtensionURIType`

### ExtensionVersionID

**XSD type:** `ExtensionVersionIDType`
