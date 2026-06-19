#[derive(Debug, Deserialize, Serialize)]
pub struct CharTwoFieldParamsType {
    #[serde(rename = "M")]
    pub m: NonZeroUsize,
}
