#[derive(Debug, Deserialize, Serialize)]
pub struct CounterSignature {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "Signature")]
    pub signature: ds::Signature,
}
