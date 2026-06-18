#[derive(Debug, Deserialize, Serialize)]
pub struct ReferenceInfo {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "@URI")]
    pub uri: Option<String>,
    #[serde(rename = "DigestMethod")]
    pub digest_method: super::ds::DigestMethod,
    #[serde(rename = "DigestValue")]
    pub digest_value: String,
}
