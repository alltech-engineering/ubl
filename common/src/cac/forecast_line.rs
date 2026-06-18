#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "FrozenDocumentIndicator")]
    pub frozen_document_indicator: Option<super::udt::IndicatorType>,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: super::cct::CodeType,
    #[serde(default, rename = "ForecastPeriod")]
    pub forecast_period: Option<Period>,
    #[serde(default, rename = "SalesItem")]
    pub sales_item: Option<SalesItem>,
}
