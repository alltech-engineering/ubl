#[derive(Debug, Deserialize, Serialize)]
pub struct IdentifierType {
    #[serde(default, rename = "@Qualifier")]
    pub qualifier: Option<QualifierType>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
