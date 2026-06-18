#[derive(Debug, Deserialize, Serialize)]
pub struct TelecommunicationsSupply {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "TelecommunicationsSupplyType")]
    pub telecommunications_supply_type: Option<super::cct::TextType>,
    #[serde(default, rename = "TelecommunicationsSupplyTypeCode")]
    pub telecommunications_supply_type_code: Option<super::cct::CodeType>,
    #[serde(rename = "PrivacyCode")]
    pub privacy_code: super::cct::CodeType,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "TotalAmount")]
    pub total_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TelecommunicationsSupplyLine")]
    pub telecommunications_supply_line: Vec<TelecommunicationsSupplyLine>,
}
