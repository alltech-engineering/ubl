#[derive(Debug, Deserialize, Serialize)]
pub struct PrimeFieldParams {
    #[serde(rename = "P")]
    pub p: String,
}
