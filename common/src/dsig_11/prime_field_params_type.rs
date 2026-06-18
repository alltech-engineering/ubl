#[derive(Debug, Deserialize, Serialize)]
pub struct PrimeFieldParamsType {
    #[serde(rename = "P")]
    pub p: String,
}
