#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentationReferencesTypeContent {
    #[serde(rename = "DocumentationReference")]
    pub documentation_reference: String,
}
