# CCT — UN/CEFACT Core Component Types

Generated from OASIS UBL 2.5 XSD annotations. 10 types.

## Types

- [Amount](#amount) — A number of monetary units specified in a currency where the unit of the currenc...
- [BinaryObject](#binaryobject) — A set of finite-length sequences of binary octets.
- [Code](#code) — A character string (letters, figures, or symbols) that for brevity and/or langua...
- [DateTime](#datetime) — A particular point in the progression of time together with the relevant supplem...
- [Identifier](#identifier) — A character string to identify and distinguish uniquely, one instance of an obje...
- [Indicator](#indicator) — A list of two mutually exclusive Boolean values that express the only possible s...
- [Measure](#measure) — A numeric value determined by measuring an object along with the specified unit ...
- [Numeric](#numeric) — Numeric information that is assigned or is determined by calculation, counting, ...
- [Quantity](#quantity) — A counted number of non-monetary units possibly including fractions.
- [Text](#text) — A character string (i.e. a finite set of characters) generally in the form of wo...

### Amount

**XSD type:** `AmountType`
**Definition:** A number of monetary units specified in a currency where the unit of the currency is explicit or implied.
**DictionaryEntryName:** Amount. Type
**Representation Term:** Amount
**Primitive Type:** decimal
**Unique ID:** UNDT000001
**Category:** CCT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `currencyID` | `xsd:normalizedString` | The currency of the amount. |
| `currencyCodeListVersionID` | `xsd:normalizedString` | The VersionID of the UN/ECE Rec9 code list. |

### BinaryObject

**XSD type:** `BinaryObjectType`
**Definition:** A set of finite-length sequences of binary octets.
**DictionaryEntryName:** Binary Object. Type
**Representation Term:** Binary Object
**Primitive Type:** binary
**Unique ID:** UNDT000002
**Category:** CCT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `format` | `xsd:string` | The format of the binary content. |
| `mimeCode` | `xsd:normalizedString` | The mime type of the binary object. |
| `encodingCode` | `xsd:normalizedString` | Specifies the decoding algorithm of the binary object. |
| `characterSetCode` | `xsd:normalizedString` | The character set of the binary object if the mime type is text. |
| `uri` | `xsd:anyURI` | The Uniform Resource Identifier that identifies where the binary object is located. |
| `filename` | `xsd:string` | The filename of the binary object. |

### Code

**XSD type:** `CodeType`
**Definition:** A character string (letters, figures, or symbols) that for brevity and/or languange independence may be used to represent or replace a definitive value or text of an attribute together with relevant supplementary information.
**DictionaryEntryName:** Code. Type
**Representation Term:** Code
**Primitive Type:** string
**Unique ID:** UNDT000007
**Category:** CCT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `listID` | `xsd:normalizedString` | The identification of a list of codes. |
| `listAgencyID` | `xsd:normalizedString` | An agency that maintains one or more lists of codes. |
| `listAgencyName` | `xsd:string` | The name of the agency that maintains the list of codes. |
| `listName` | `xsd:string` | The name of a list of codes. |
| `listVersionID` | `xsd:normalizedString` | The version of the list of codes. |
| `name` | `xsd:string` | The textual equivalent of the code content component. |
| `languageID` | `xsd:language` | The identifier of the language used in the code name. |
| `listURI` | `xsd:anyURI` | The Uniform Resource Identifier that identifies where the code list is located. |
| `listSchemeURI` | `xsd:anyURI` | The Uniform Resource Identifier that identifies where the code list scheme is located. |

### DateTime

**XSD type:** `DateTimeType`
**Definition:** A particular point in the progression of time together with the relevant supplementary information.
**DictionaryEntryName:** Date Time. Type
**Representation Term:** Date Time
**Primitive Type:** string
**Unique ID:** UNDT000008
**Category:** CCT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `format` | `xsd:string` | The format of the date time content |

### Identifier

**XSD type:** `IdentifierType`
**Definition:** A character string to identify and distinguish uniquely, one instance of an object in an identification scheme from all other objects in the same scheme together with relevant supplementary information.
**DictionaryEntryName:** Identifier. Type
**Representation Term:** Identifier
**Primitive Type:** string
**Unique ID:** UNDT000011
**Category:** CCT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `schemeID` | `xsd:normalizedString` | The identification of the identification scheme. |
| `schemeName` | `xsd:string` | The name of the identification scheme. |
| `schemeAgencyID` | `xsd:normalizedString` | The identification of the agency that maintains the identification scheme. |
| `schemeAgencyName` | `xsd:string` | The name of the agency that maintains the identification scheme. |
| `schemeVersionID` | `xsd:normalizedString` | The version of the identification scheme. |
| `schemeDataURI` | `xsd:anyURI` | The Uniform Resource Identifier that identifies where the identification scheme data is located. |
| `schemeURI` | `xsd:anyURI` | The Uniform Resource Identifier that identifies where the identification scheme is located. |

### Indicator

**XSD type:** `IndicatorType`
**Definition:** A list of two mutually exclusive Boolean values that express the only possible states of a Property.
**DictionaryEntryName:** Indicator. Type
**Representation Term:** Indicator
**Primitive Type:** string
**Unique ID:** UNDT000012
**Category:** CCT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `format` | `xsd:string` | Whether the indicator is numeric, textual or binary. |

### Measure

**XSD type:** `MeasureType`
**Definition:** A numeric value determined by measuring an object along with the specified unit of measure.
**DictionaryEntryName:** Measure. Type
**Representation Term:** Measure
**Primitive Type:** decimal
**Unique ID:** UNDT000013
**Category:** CCT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `unitCode` | `xsd:normalizedString` | The type of unit of measure. |
| `unitCodeListVersionID` | `xsd:normalizedString` | The version of the measure unit code list. |

### Numeric

**XSD type:** `NumericType`
**Definition:** Numeric information that is assigned or is determined by calculation, counting, or sequencing. It does not require a unit of quantity or unit of measure.
**DictionaryEntryName:** Numeric. Type
**Representation Term:** Numeric
**Primitive Type:** string
**Unique ID:** UNDT000014
**Category:** CCT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `format` | `xsd:string` | Whether the number is an integer, decimal, real number or percentage. |

### Quantity

**XSD type:** `QuantityType`
**Definition:** A counted number of non-monetary units possibly including fractions.
**DictionaryEntryName:** Quantity. Type
**Representation Term:** Quantity
**Primitive Type:** decimal
**Unique ID:** UNDT000018
**Category:** CCT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `unitCode` | `xsd:normalizedString` | The unit of the quantity |
| `unitCodeListID` | `xsd:normalizedString` | The quantity unit code list. |
| `unitCodeListAgencyID` | `xsd:normalizedString` | The identification of the agency that maintains the quantity unit code list |
| `unitCodeListAgencyName` | `xsd:string` | The name of the agency which maintains the quantity unit code list. |

### Text

**XSD type:** `TextType`
**Definition:** A character string (i.e. a finite set of characters) generally in the form of words of a language.
**DictionaryEntryName:** Text. Type
**Representation Term:** Text
**Primitive Type:** string
**Unique ID:** UNDT000019
**Category:** CCT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `languageID` | `xsd:language` | The identifier of the language used in the content component. |
| `languageLocaleID` | `xsd:normalizedString` | The identification of the locale of the language. |
