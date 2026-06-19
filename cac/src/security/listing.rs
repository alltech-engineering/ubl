#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityListing {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(rename = "MarketName")]
    pub market_name: cct::Text,
    #[serde(default, rename = "MarketCode")]
    pub market_code: Option<cct::Code>,
}
