#[derive(Debug, Deserialize, Serialize)]
pub struct PerformanceDataLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(rename = "PerformanceValueQuantity")]
    pub performance_value_quantity: cct::Quantity,
    #[serde(rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: cct::Code,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
    #[serde(default, rename = "Item")]
    pub item: Option<Item>,
}
