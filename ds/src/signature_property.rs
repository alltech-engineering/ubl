#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureProperty {
    #[serde(rename = "@Target")]
    pub target: String,
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "$value")]
    pub content: Vec<SignaturePropertyTypeContent>,
}
