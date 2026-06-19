#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the metadata of a specific business document based on any document format (e.g.
/// UBL, EDIFACT, ...).
///
/// UBL Dictionary Entry Name: `Document Metadata. Details`
///
/// Generated from XSD type `DocumentMetadataType`.
pub struct DocumentMetadata {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the document.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// An identifier for the document format (e.g. standard business vocabularies).
    #[serde(rename = "FormatID")]
    pub format_id: cct::Identifier,
/// An identifier for a precise version of a document format.
    #[serde(rename = "VersionID")]
    pub version_id: cct::Identifier,
/// The Uniform Resource Identifier (URI) of a schema definition for the business document (e.g. a
/// namespace URI for XML schemas, a message ID for non-xml legacy documents).
    #[serde(default, rename = "SchemaURI")]
    pub schema_uri: Option<cct::Identifier>,
/// The type of document, expressed as a code.
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: Option<cct::Code>,
}
