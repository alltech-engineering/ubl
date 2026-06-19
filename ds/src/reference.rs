#[derive(Debug, Deserialize, Serialize)]
pub struct Reference {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "@URI")]
    pub uri: Option<String>,
    #[serde(default, rename = "@Type")]
    pub type_: Option<String>,
    #[serde(default, rename = "Transforms")]
    pub transforms: Option<Transforms>,
    #[serde(rename = "DigestMethod")]
    pub digest_method: DigestMethod,
    #[serde(rename = "DigestValue")]
    pub digest_value: String,
}
