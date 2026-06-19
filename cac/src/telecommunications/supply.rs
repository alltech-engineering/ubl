#[derive(Debug, Deserialize, Serialize)]
pub struct TelecommunicationsSupply {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "TelecommunicationsSupplyType")]
    pub telecommunications_supply_type: Option<cct::Text>,
    #[serde(default, rename = "TelecommunicationsSupplyTypeCode")]
    pub telecommunications_supply_type_code: Option<cct::Code>,
    #[serde(rename = "PrivacyCode")]
    pub privacy_code: cct::Code,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "TotalAmount")]
    pub total_amount: Option<cct::Amount>,
    #[serde(default, rename = "TelecommunicationsSupplyLine")]
    pub telecommunications_supply_line: Vec<TelecommunicationsSupplyLine>,
}
