#[derive(Debug, Deserialize, Serialize)]
pub enum EcKeyValueTypeContent {
    #[serde(rename = "ECParameters")]
    EcParameters(EcParametersType),
    #[serde(rename = "NamedCurve")]
    NamedCurve(NamedCurveType),
    #[serde(rename = "PublicKey")]
    PublicKey(String),
}
