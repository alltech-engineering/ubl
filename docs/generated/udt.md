# UDT — UN/CEFACT Unqualified Data Types

Generated from OASIS UBL 2.5 XSD annotations. 20 types.

## Types

- [Amount](#amount) — A number of monetary units specified using a given unit of currency.
- [BinaryObject](#binaryobject) — A set of finite-length sequences of binary octets.
- [Graphic](#graphic) — A diagram, graph, mathematical curve, or similar representation.
- [Picture](#picture) — A diagram, graph, mathematical curve, or similar representation.
- [Sound](#sound) — An audio representation.
- [Video](#video) — A video representation.
- [Code](#code) — A character string (letters, figures, or symbols) that for brevity and/or langua...
- [DateTime](#datetime) — A particular point in the progression of time, together with relevant supplement...
- [Date](#date) — One calendar day according the Gregorian calendar.
- [Time](#time) — An instance of time that occurs every day.
- [Identifier](#identifier) — A character string to identify and uniquely distinguish one instance of an objec...
- [Indicator](#indicator) — A list of two mutually exclusive Boolean values that express the only possible s...
- [Measure](#measure) — A numeric value determined by measuring an object using a specified unit of meas...
- [Numeric](#numeric) — Numeric information that is assigned or is determined by calculation, counting, ...
- [Value](#value) — Numeric information that is assigned or is determined by calculation, counting, ...
- [Percent](#percent) — Numeric information that is assigned or is determined by calculation, counting, ...
- [Rate](#rate) — A numeric expression of a rate that is assigned or is determined by calculation,...
- [Quantity](#quantity) — A counted number of non-monetary units, possibly including a fractional part.
- [Text](#text) — A character string (i.e. a finite set of characters), generally in the form of w...
- [Name](#name) — A character string that constitutes the distinctive designation of a person, pla...

### Amount

**XSD type:** `AmountType`
**Definition:** A number of monetary units specified using a given unit of currency.
**DictionaryEntryName:** Amount. Type
**Representation Term:** Amount
**Unique ID:** BDNDRUDT000001
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `currencyID` | `xsd:normalizedString` | The currency of the amount. |
| `currencyCodeListVersionID` | `xsd:normalizedString` | (Deprecated) The VersionID of the UN/ECE Rec9 code list. |

### BinaryObject

**XSD type:** `BinaryObjectType`
**Definition:** A set of finite-length sequences of binary octets.
**DictionaryEntryName:** Binary Object. Type
**Representation Term:** Binary Object
**Primitive Type:** binary
**Unique ID:** BDNDRUDT000002
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `mimeCode` | `xsd:normalizedString` | The mime type of the binary object. |
| `format` | `xsd:string` | (Deprecated) The format of the binary content. |
| `encodingCode` | `xsd:normalizedString` | (Deprecated) Specifies the decoding algorithm of the binary object. |
| `characterSetCode` | `xsd:normalizedString` | (Deprecated) The character set of the binary object if the mime type is text. |
| `uri` | `xsd:anyURI` | (Deprecated) The Uniform Resource Identifier that identifies where the binary object is located. |
| `filename` | `xsd:string` | (Deprecated) The filename of the binary object. |

### Graphic

**XSD type:** `GraphicType`
**Definition:** A diagram, graph, mathematical curve, or similar representation.
**DictionaryEntryName:** Graphic. Type
**Representation Term:** Graphic
**Primitive Type:** binary
**Unique ID:** BDNDRUDT000003
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `mimeCode` | `xsd:normalizedString` | The mime type of the graphic object. |
| `format` | `xsd:string` | (Deprecated) The format of the binary content. |
| `encodingCode` | `xsd:normalizedString` | (Deprecated) Specifies the decoding algorithm of the binary object. |
| `characterSetCode` | `xsd:normalizedString` | (Deprecated) The character set of the binary object if the mime type is text. |
| `uri` | `xsd:anyURI` | (Deprecated) The Uniform Resource Identifier that identifies where the binary object is located. |
| `filename` | `xsd:string` | (Deprecated) The filename of the binary object. |

### Picture

**XSD type:** `PictureType`
**Definition:** A diagram, graph, mathematical curve, or similar representation.
**DictionaryEntryName:** Picture. Type
**Representation Term:** Picture
**Primitive Type:** binary
**Unique ID:** BDNDRUDT000004
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `mimeCode` | `xsd:normalizedString` | The mime type of the picture object. |
| `format` | `xsd:string` | (Deprecated) The format of the binary content. |
| `encodingCode` | `xsd:normalizedString` | (Deprecated) Specifies the decoding algorithm of the binary object. |
| `characterSetCode` | `xsd:normalizedString` | (Deprecated) The character set of the binary object if the mime type is text. |
| `uri` | `xsd:anyURI` | (Deprecated) The Uniform Resource Identifier that identifies where the binary object is located. |
| `filename` | `xsd:string` | (Deprecated) The filename of the binary object. |

### Sound

**XSD type:** `SoundType`
**Definition:** An audio representation.
**DictionaryEntryName:** Sound. Type
**Representation Term:** Sound
**Primitive Type:** binary
**Unique ID:** BDNDRUDT000005
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `mimeCode` | `xsd:normalizedString` | The mime type of the sound object. |
| `format` | `xsd:string` | (Deprecated) The format of the binary content. |
| `encodingCode` | `xsd:normalizedString` | (Deprecated) Specifies the decoding algorithm of the binary object. |
| `characterSetCode` | `xsd:normalizedString` | (Deprecated) The character set of the binary object if the mime type is text. |
| `uri` | `xsd:anyURI` | (Deprecated) The Uniform Resource Identifier that identifies where the binary object is located. |
| `filename` | `xsd:string` | (Deprecated) The filename of the binary object. |

### Video

**XSD type:** `VideoType`
**Definition:** A video representation.
**DictionaryEntryName:** Video. Type
**Representation Term:** Video
**Primitive Type:** binary
**Unique ID:** BDNDRUDT000006
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `mimeCode` | `xsd:normalizedString` | The mime type of the video object. |
| `format` | `xsd:string` | (Deprecated) The format of the binary content. |
| `encodingCode` | `xsd:normalizedString` | (Deprecated) Specifies the decoding algorithm of the binary object. |
| `characterSetCode` | `xsd:normalizedString` | (Deprecated) The character set of the binary object if the mime type is text. |
| `uri` | `xsd:anyURI` | (Deprecated) The Uniform Resource Identifier that identifies where the binary object is located. |
| `filename` | `xsd:string` | (Deprecated) The filename of the binary object. |

### Code

**XSD type:** `CodeType`
**Definition:** A character string (letters, figures, or symbols) that for brevity and/or language independence may be used to represent or replace a definitive value or text of an attribute, together with relevant supplementary information.
**DictionaryEntryName:** Code. Type
**Representation Term:** Code
**Primitive Type:** string
**Unique ID:** BDNDRUDT000007
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `listAgencyID` | `xsd:normalizedString` | (Deprecated) An agency that maintains one or more lists of codes. |
| `listAgencyName` | `xsd:string` | (Deprecated) The name of the agency that maintains the list of codes. |
| `listName` | `xsd:string` | (Deprecated) The name of a list of codes. |
| `listVersionID` | `xsd:normalizedString` | (Deprecated) The version of the list of codes. |
| `name` | `xsd:string` | (Deprecated) The textual equivalent of the code content component. |
| `languageID` | `xsd:language` | (Deprecated) The identifier of the language used in the code name. |
| `listURI` | `xsd:anyURI` | (Deprecated) The Uniform Resource Identifier that identifies where the code list is located. |
| `listSchemeURI` | `xsd:anyURI` | (Deprecated) The Uniform Resource Identifier that identifies where the code list scheme is located. |

### DateTime

**XSD type:** `DateTimeType`
**Definition:** A particular point in the progression of time, together with relevant supplementary information.
**DictionaryEntryName:** Date Time. Type
**Representation Term:** Date Time
**Primitive Type:** string
**Unique ID:** BDNDRUDT000008
**Category:** UDT
**Version:** 1.0

### Date

**XSD type:** `DateType`
**Definition:** One calendar day according the Gregorian calendar.
**DictionaryEntryName:** Date. Type
**Representation Term:** Date
**Primitive Type:** string
**Unique ID:** BDNDRUDT000009
**Category:** UDT
**Version:** 1.0

### Time

**XSD type:** `TimeType`
**Definition:** An instance of time that occurs every day.
**DictionaryEntryName:** Time. Type
**Representation Term:** Time
**Primitive Type:** string
**Unique ID:** BDNDRUDT0000010
**Category:** UDT
**Version:** 1.0

### Identifier

**XSD type:** `IdentifierType`
**Definition:** A character string to identify and uniquely distinguish one instance of an object in an identification scheme from all other objects in the same scheme, together with relevant supplementary information.
**DictionaryEntryName:** Identifier. Type
**Representation Term:** Identifier
**Primitive Type:** string
**Unique ID:** BDNDRUDT0000011
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `schemeName` | `xsd:string` | (Deprecated) The name of the identification scheme. |
| `schemeAgencyID` | `xsd:normalizedString` | (Deprecated) The identification of the agency that maintains the identification scheme. |
| `schemeAgencyName` | `xsd:string` | (Deprecated) The name of the agency that maintains the identification scheme. |
| `schemeVersionID` | `xsd:normalizedString` | (Deprecated) The version of the identification scheme. |
| `schemeDataURI` | `xsd:anyURI` | (Deprecated) The Uniform Resource Identifier that identifies where the identification scheme data is located. |
| `schemeURI` | `xsd:anyURI` | (Deprecated) The Uniform Resource Identifier that identifies where the identification scheme is located. |

### Indicator

**XSD type:** `IndicatorType`
**Definition:** A list of two mutually exclusive Boolean values that express the only possible states of a property.
**DictionaryEntryName:** Indicator. Type
**Representation Term:** Indicator
**Primitive Type:** string
**Unique ID:** BDNDRUDT0000012
**Category:** UDT
**Version:** 1.0

### Measure

**XSD type:** `MeasureType`
**Definition:** A numeric value determined by measuring an object using a specified unit of measure.
**DictionaryEntryName:** Measure. Type
**Representation Term:** Measure
**Primitive Type:** decimal
**Unique ID:** BDNDRUDT0000013
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `unitCode` | `xsd:normalizedString` | The type of unit of measure. |
| `unitCodeListVersionID` | `xsd:normalizedString` | (Deprecated) The version of the measure unit code list. |

### Numeric

**XSD type:** `NumericType`
**Definition:** Numeric information that is assigned or is determined by calculation, counting, or sequencing. It does not require a unit of quantity or unit of measure.
**DictionaryEntryName:** Numeric. Type
**Representation Term:** Numeric
**Primitive Type:** string
**Unique ID:** BDNDRUDT0000014
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `format` | `xsd:string` | (Deprecated) Whether the number is an integer, decimal, real number or percentage. |

### Value

**XSD type:** `ValueType`
**Definition:** Numeric information that is assigned or is determined by calculation, counting, or sequencing. It does not require a unit of quantity or unit of measure.
**DictionaryEntryName:** Value. Type
**Representation Term:** Value
**Primitive Type:** string
**Unique ID:** BDNDRUDT0000015
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `format` | `xsd:string` | (Deprecated) Whether the number is an integer, decimal, real number or percentage. |

### Percent

**XSD type:** `PercentType`
**Definition:** Numeric information that is assigned or is determined by calculation, counting, or sequencing and is expressed as a percentage. It does not require a unit of quantity or unit of measure.
**DictionaryEntryName:** Percent. Type
**Representation Term:** Percent
**Primitive Type:** string
**Unique ID:** BDNDRUDT0000016
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `format` | `xsd:string` | (Deprecated) Whether the number is an integer, decimal, real number or percentage. |

### Rate

**XSD type:** `RateType`
**Definition:** A numeric expression of a rate that is assigned or is determined by calculation, counting, or sequencing. It does not require a unit of quantity or unit of measure.
**DictionaryEntryName:** Rate. Type
**Representation Term:** Rate
**Primitive Type:** string
**Unique ID:** BDNDRUDT0000017
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `format` | `xsd:string` | (Deprecated) Whether the number is an integer, decimal, real number or percentage. |

### Quantity

**XSD type:** `QuantityType`
**Definition:** A counted number of non-monetary units, possibly including a fractional part.
**DictionaryEntryName:** Quantity. Type
**Representation Term:** Quantity
**Primitive Type:** decimal
**Unique ID:** BDNDRUDT0000018
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `unitCodeListID` | `xsd:normalizedString` | (Deprecated) The quantity unit code list. |
| `unitCodeListAgencyID` | `xsd:normalizedString` | (Deprecated) The identification of the agency that maintains the quantity unit code list |
| `unitCodeListAgencyName` | `xsd:string` | (Deprecated) The name of the agency which maintains the quantity unit code list. |

### Text

**XSD type:** `TextType`
**Definition:** A character string (i.e. a finite set of characters), generally in the form of words of a language.
**DictionaryEntryName:** Text. Type
**Representation Term:** Text
**Primitive Type:** string
**Unique ID:** BDNDRUDT0000019
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `languageLocaleID` | `xsd:normalizedString` | (Deprecated) The identification of the locale of the language. |

### Name

**XSD type:** `NameType`
**Definition:** A character string that constitutes the distinctive designation of a person, place, thing or concept.
**DictionaryEntryName:** Name. Type
**Representation Term:** Name
**Primitive Type:** string
**Unique ID:** BDNDRUDT0000020
**Category:** UDT
**Version:** 1.0

**Attributes:**

| Attribute | Type | Definition |
|-----------|------|------------|
| `languageLocaleID` | `xsd:normalizedString` | (Deprecated) The identification of the locale of the language. |
