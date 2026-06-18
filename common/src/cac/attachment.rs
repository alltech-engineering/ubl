#[derive(Debug, Deserialize, Serialize)]
pub struct Attachment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "EmbeddedDocumentBinaryObject")]
    pub embedded_document_binary_object: Option<super::cct::BinaryObjectType>,
    #[serde(default, rename = "EmbeddedDocument")]
    pub embedded_document: Option<super::cct::TextType>,
    #[serde(default, rename = "FileName")]
    pub file_name: Option<super::cct::TextType>,
    #[serde(default, rename = "ExternalReference")]
    pub external_reference: Option<ExternalReference>,
}
