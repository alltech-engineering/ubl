#[derive(Debug, Deserialize, Serialize)]
pub struct CurveType {
    #[serde(rename = "A")]
    pub a: String,
    #[serde(rename = "B")]
    pub b: String,
}
