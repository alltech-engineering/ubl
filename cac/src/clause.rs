#[derive(Debug, Deserialize, Serialize)]
pub struct Clause {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Content")]
    pub content: Vec<cct::Text>,
}
