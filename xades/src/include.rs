#[derive(Debug, Deserialize, Serialize)]
pub struct Include {
    #[serde(rename = "@URI")]
    pub uri: String,
    #[serde(default, rename = "@referencedData")]
    pub referenced_data: Option<bool>,
}
