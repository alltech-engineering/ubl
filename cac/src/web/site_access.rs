#[derive(Debug, Deserialize, Serialize)]
/// A class to describe access to a web site.
///
/// UBL Dictionary Entry Name: `Web Site Access. Details`
///
/// Generated from XSD type `WebSiteAccessType`.
pub struct WebSiteAccess {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The Uniform Resource Identifier (URI) for this web site; i.e., its Uniform Resource Locator (URL).
    #[serde(default, rename = "URI")]
    pub uri: Option<cct::Identifier>,
/// A password to the web site.
    #[serde(rename = "Password")]
    pub password: cct::Text,
/// Text describing login details.
    #[serde(rename = "Login")]
    pub login: cct::Text,
}
