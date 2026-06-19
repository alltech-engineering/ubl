#[derive(Debug, Deserialize, Serialize)]
pub struct NamedCurve {
    #[serde(rename = "@URI")]
    pub uri: String,
}
