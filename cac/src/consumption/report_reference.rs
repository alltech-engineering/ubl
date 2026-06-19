#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionReportReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ConsumptionReportID")]
    pub consumption_report_id: cct::Identifier,
    #[serde(default, rename = "ConsumptionType")]
    pub consumption_type: Option<cct::Text>,
    #[serde(default, rename = "ConsumptionTypeCode")]
    pub consumption_type_code: Option<cct::Code>,
    #[serde(rename = "TotalConsumedQuantity")]
    pub total_consumed_quantity: cct::Quantity,
    #[serde(rename = "Period")]
    pub period: crate::Period,
}
