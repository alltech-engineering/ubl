#[derive(Debug, Deserialize, Serialize)]
/// A class to define a reference to a line in a document.
///
/// UBL Dictionary Entry Name: `Line Reference. Details`
///
/// Generated from XSD type `LineReferenceType`.
pub struct LineReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// Identifies the referenced line in the document.
    #[serde(rename = "LineID")]
    pub line_id: cct::Identifier,
/// A universally unique identifier for this line reference.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// A code signifying the status of the referenced line with respect to its original state.
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: Option<cct::Code>,
/// A reference to the document containing the referenced line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Option<crate::DocumentReference>,
}
