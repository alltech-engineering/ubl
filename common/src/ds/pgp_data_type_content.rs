#[derive(Debug, Deserialize, Serialize)]
pub enum PgpDataTypeContent {
    #[serde(rename = "PGPKeyID")]
    PgpKeyId(String),
    #[serde(rename = "PGPKeyPacket")]
    PgpKeyPacket(String),
    #[serde(rename = "any25")]
    Any(String),
    #[serde(rename = "any27")]
    Any2(String),
}
