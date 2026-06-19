#[derive(Debug, Deserialize, Serialize)]
pub enum EcKeyValueTypeContent {
    #[serde(rename = "ECParameters")]
    EcParameters(EcParameters),
    #[serde(rename = "NamedCurve")]
    NamedCurve(NamedCurve),
    #[serde(rename = "PublicKey")]
    PublicKey(String),
}
