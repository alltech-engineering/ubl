#[derive(Debug, Deserialize, Serialize)]
pub struct PnBFieldParamsType {
    #[serde(rename = "M")]
    pub m: NonZeroUsize,
    #[serde(rename = "K1")]
    pub k1: NonZeroUsize,
    #[serde(rename = "K2")]
    pub k2: NonZeroUsize,
    #[serde(rename = "K3")]
    pub k3: NonZeroUsize,
}
