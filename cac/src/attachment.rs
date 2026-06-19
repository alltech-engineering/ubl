#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an attached document. An attachment can refer to an external document or be
/// included with the document being exchanged.
///
/// UBL Dictionary Entry Name: `Attachment. Details`
///
/// Generated from XSD type `AttachmentType`.
pub struct Attachment {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A binary large object containing an attached document.
    #[serde(default, rename = "EmbeddedDocumentBinaryObject")]
    pub embedded_document_binary_object: Option<cct::BinaryObject>,
/// A clear text object containing an attached document.
    #[serde(default, rename = "EmbeddedDocument")]
    pub embedded_document: Option<cct::Text>,
/// The filename of the attachment.
    #[serde(default, rename = "FileName")]
    pub file_name: Option<cct::Text>,
/// A reference to an attached document that is external to the document(s) being exchanged.
    #[serde(default, rename = "ExternalReference")]
    pub external_reference: Option<ExternalReference>,
}
