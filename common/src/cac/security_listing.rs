#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityListing {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(rename = "MarketName")]
    pub market_name: super::cct::TextType,
    #[serde(default, rename = "MarketCode")]
    pub market_code: Option<super::cct::CodeType>,
}
