#[derive(Debug, Deserialize, Serialize)]
pub struct Legislation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Title")]
    pub title: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "JurisdictionLevel")]
    pub jurisdiction_level: Vec<super::cct::TextType>,
    #[serde(default, rename = "Article")]
    pub article: Vec<super::cct::TextType>,
    #[serde(default, rename = "URI")]
    pub uri: Vec<super::cct::IdentifierType>,
    #[serde(default, rename = "Language")]
    pub language: Vec<Language>,
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: Vec<Address>,
}
