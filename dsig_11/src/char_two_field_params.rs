#[derive(Debug, Deserialize, Serialize)]
pub struct CharTwoFieldParams {
    #[serde(rename = "M")]
    pub m: NonZeroUsize,
}
