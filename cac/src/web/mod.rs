use serde::{Deserialize, Serialize};


include!("site_access.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a web site.
///
/// UBL Dictionary Entry Name: `Web Site. Details`
///
/// Generated from XSD type `WebSiteType`.
pub struct WebSite {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for a specific web site.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The common name of the web site.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// Text describing the web site.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A code that specifies the type web site.
    #[serde(default, rename = "WebSiteTypeCode")]
    pub web_site_type_code: Option<cct::Code>,
/// The Uniform Resource Identifier (URI) of the web site; i.e., its Uniform Resource Locator (URL).
    #[serde(rename = "URI")]
    pub uri: cct::Identifier,
/// Access information for the website (e.g. guest credentials).
    #[serde(default, rename = "WebSiteAccess")]
    pub web_site_access: Vec<WebSiteAccess>,
}
