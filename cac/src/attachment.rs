#[derive(Debug, Deserialize, Serialize)]
pub struct Attachment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "EmbeddedDocumentBinaryObject")]
    pub embedded_document_binary_object: Option<cct::BinaryObject>,
    #[serde(default, rename = "EmbeddedDocument")]
    pub embedded_document: Option<cct::Text>,
    #[serde(default, rename = "FileName")]
    pub file_name: Option<cct::Text>,
    #[serde(default, rename = "ExternalReference")]
    pub external_reference: Option<ExternalReference>,
}
