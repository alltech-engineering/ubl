#[derive(Debug, Deserialize, Serialize)]
pub struct TenderRequirement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Name")]
    pub name: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "TemplateDocumentReference")]
    pub template_document_reference: Option<DocumentReference>,
}
