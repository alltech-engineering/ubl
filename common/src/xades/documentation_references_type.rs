#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentationReferencesType {
    #[serde(rename = "$value")]
    pub content: Vec<DocumentationReferencesTypeContent>,
}
