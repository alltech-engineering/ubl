use serde::{Deserialize, Serialize};


include!("site_access.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct WebSite {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "WebSiteTypeCode")]
    pub web_site_type_code: Option<cct::Code>,
    #[serde(rename = "URI")]
    pub uri: cct::Identifier,
    #[serde(default, rename = "WebSiteAccess")]
    pub web_site_access: Vec<WebSiteAccess>,
}
