#[derive(Debug, Deserialize, Serialize)]
pub enum KeyValueTypeContent {
    #[serde(rename = "DSAKeyValue")]
    DsaKeyValue(DsaKeyValue),
    #[serde(rename = "RSAKeyValue")]
    RsaKeyValue(RsaKeyValue),
    #[serde(rename = "any17")]
    Any(String),
}
