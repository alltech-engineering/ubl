#[derive(Debug, Deserialize, Serialize)]
pub struct TenderRequirement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "TemplateDocumentReference")]
    pub template_document_reference: Option<crate::DocumentReference>,
}
