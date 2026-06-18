#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionReportReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ConsumptionReportID")]
    pub consumption_report_id: super::cct::IdentifierType,
    #[serde(default, rename = "ConsumptionType")]
    pub consumption_type: Option<super::cct::TextType>,
    #[serde(default, rename = "ConsumptionTypeCode")]
    pub consumption_type_code: Option<super::cct::CodeType>,
    #[serde(rename = "TotalConsumedQuantity")]
    pub total_consumed_quantity: super::cct::QuantityType,
    #[serde(rename = "Period")]
    pub period: Period,
}
