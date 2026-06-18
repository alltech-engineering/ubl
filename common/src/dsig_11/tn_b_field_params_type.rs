#[derive(Debug, Deserialize, Serialize)]
pub struct TnBFieldParamsType {
    #[serde(rename = "M")]
    pub m: NonZeroUsize,
    #[serde(rename = "K")]
    pub k: NonZeroUsize,
}
