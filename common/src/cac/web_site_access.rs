#[derive(Debug, Deserialize, Serialize)]
pub struct WebSiteAccess {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "URI")]
    pub uri: Option<super::cct::IdentifierType>,
    #[serde(rename = "Password")]
    pub password: super::cct::TextType,
    #[serde(rename = "Login")]
    pub login: super::cct::TextType,
}
