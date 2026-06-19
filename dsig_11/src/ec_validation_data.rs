#[derive(Debug, Deserialize, Serialize)]
pub struct EcValidationData {
    #[serde(rename = "@hashAlgorithm")]
    pub hash_algorithm: String,
    #[serde(rename = "seed")]
    pub seed: String,
}
