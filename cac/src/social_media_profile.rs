#[derive(Debug, Deserialize, Serialize)]
pub struct SocialMediaProfile {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "SocialMediaTypeCode")]
    pub social_media_type_code: Option<cct::Code>,
    #[serde(rename = "URI")]
    pub uri: cct::Identifier,
}
