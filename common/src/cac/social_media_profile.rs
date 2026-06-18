#[derive(Debug, Deserialize, Serialize)]
pub struct SocialMediaProfile {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "SocialMediaTypeCode")]
    pub social_media_type_code: Option<super::cct::CodeType>,
    #[serde(rename = "URI")]
    pub uri: super::cct::IdentifierType,
}
