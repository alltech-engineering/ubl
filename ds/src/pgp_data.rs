#[derive(Debug, Deserialize, Serialize)]
pub struct PgpData {
    #[serde(rename = "$value")]
    pub content: Vec<PgpDataTypeContent>,
}
