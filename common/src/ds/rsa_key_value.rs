#[derive(Debug, Deserialize, Serialize)]
pub struct RsaKeyValue {
    #[serde(rename = "Modulus")]
    pub modulus: String,
    #[serde(rename = "Exponent")]
    pub exponent: String,
}
