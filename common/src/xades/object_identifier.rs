#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectIdentifier {
    #[serde(rename = "Identifier")]
    pub identifier: IdentifierType,
    #[serde(default, rename = "Description")]
    pub description: Option<String>,
    #[serde(default, rename = "DocumentationReferences")]
    pub documentation_references: Option<DocumentationReferencesType>,
}
