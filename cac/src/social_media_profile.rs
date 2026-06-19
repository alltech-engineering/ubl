#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a social media profile.
///
/// UBL Dictionary Entry Name: `Social Media Profile. Details`
///
/// Generated from XSD type `SocialMediaProfileType`.
pub struct SocialMediaProfile {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for a specific social media.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The common name of the social media.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// A code that specifies the type of social media.
    #[serde(default, rename = "SocialMediaTypeCode")]
    pub social_media_type_code: Option<cct::Code>,
/// The Uniform Resource Identifier (URI) of a party profile in the social media; i.e., its Uniform
/// Resource Locator (URL).
    #[serde(rename = "URI")]
    pub uri: cct::Identifier,
}
