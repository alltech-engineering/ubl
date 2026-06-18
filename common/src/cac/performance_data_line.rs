#[derive(Debug, Deserialize, Serialize)]
pub struct PerformanceDataLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(rename = "PerformanceValueQuantity")]
    pub performance_value_quantity: super::cct::QuantityType,
    #[serde(rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: super::cct::CodeType,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
    #[serde(default, rename = "Item")]
    pub item: Option<Item>,
}
