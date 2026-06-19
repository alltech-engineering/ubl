#[derive(Debug, Deserialize, Serialize)]
pub struct RetrievalMethod {
    #[serde(default, rename = "@URI")]
    pub uri: Option<String>,
    #[serde(default, rename = "@Type")]
    pub type_: Option<String>,
    #[serde(default, rename = "Transforms")]
    pub transforms: Option<Transforms>,
}
