#[derive(Debug, Deserialize, Serialize)]
pub struct CrlIdentifierType {
    #[serde(default, rename = "@URI")]
    pub uri: Option<String>,
    #[serde(rename = "Issuer")]
    pub issuer: String,
    #[serde(rename = "IssueTime")]
    pub issue_time: String,
    #[serde(default, rename = "Number")]
    pub number: Option<i32>,
}
