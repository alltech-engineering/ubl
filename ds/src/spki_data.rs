#[derive(Debug, Deserialize, Serialize)]
pub struct SpkiData {
    #[serde(rename = "$value")]
    pub content: Vec<SpkiDataTypeContent>,
}
