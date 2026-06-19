#[derive(Debug, Deserialize, Serialize)]
pub struct TnBFieldParams {
    #[serde(rename = "M")]
    pub m: NonZeroUsize,
    #[serde(rename = "K")]
    pub k: NonZeroUsize,
}
