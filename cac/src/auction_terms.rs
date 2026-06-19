#[derive(Debug, Deserialize, Serialize)]
pub struct AuctionTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "AuctionConstraintIndicator")]
    pub auction_constraint_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "JustificationDescription")]
    pub justification_description: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "ProcessDescription")]
    pub process_description: Vec<cct::Text>,
    #[serde(default, rename = "ConditionsDescription")]
    pub conditions_description: Vec<cct::Text>,
    #[serde(default, rename = "ElectronicDeviceDescription")]
    pub electronic_device_description: Vec<cct::Text>,
    #[serde(default, rename = "AuctionURI")]
    pub auction_uri: Option<cct::Identifier>,
}
