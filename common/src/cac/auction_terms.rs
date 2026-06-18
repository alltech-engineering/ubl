#[derive(Debug, Deserialize, Serialize)]
pub struct AuctionTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "AuctionConstraintIndicator")]
    pub auction_constraint_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "JustificationDescription")]
    pub justification_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ProcessDescription")]
    pub process_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ConditionsDescription")]
    pub conditions_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ElectronicDeviceDescription")]
    pub electronic_device_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "AuctionURI")]
    pub auction_uri: Option<super::cct::IdentifierType>,
}
