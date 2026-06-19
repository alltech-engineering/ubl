#[derive(Debug, Deserialize, Serialize)]
pub struct Legislation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Title")]
    pub title: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "JurisdictionLevel")]
    pub jurisdiction_level: Vec<cct::Text>,
    #[serde(default, rename = "Article")]
    pub article: Vec<cct::Text>,
    #[serde(default, rename = "URI")]
    pub uri: Vec<cct::Identifier>,
    #[serde(default, rename = "Language")]
    pub language: Vec<Language>,
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: Vec<Address>,
}
