#[derive(Debug, Deserialize, Serialize)]
pub struct WebSiteAccess {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "URI")]
    pub uri: Option<cct::Identifier>,
    #[serde(rename = "Password")]
    pub password: cct::Text,
    #[serde(rename = "Login")]
    pub login: cct::Text,
}
