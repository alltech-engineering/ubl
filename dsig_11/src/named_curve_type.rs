#[derive(Debug, Deserialize, Serialize)]
pub struct NamedCurveType {
    #[serde(rename = "@URI")]
    pub uri: String,
}
