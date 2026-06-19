#[derive(Debug, Deserialize, Serialize)]
pub struct Curve {
    #[serde(rename = "A")]
    pub a: String,
    #[serde(rename = "B")]
    pub b: String,
}
