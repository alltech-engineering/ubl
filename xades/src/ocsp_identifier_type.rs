#[derive(Debug, Deserialize, Serialize)]
pub struct OcspIdentifierType {
    #[serde(default, rename = "@URI")]
    pub uri: Option<String>,
    #[serde(rename = "ResponderID")]
    pub responder_id: ResponderIdType,
    #[serde(rename = "ProducedAt")]
    pub produced_at: String,
}
