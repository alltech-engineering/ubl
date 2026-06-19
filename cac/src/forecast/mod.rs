use serde::{Deserialize, Serialize};

include!("exception.rs");
include!("revision_line.rs");
include!("exception_criterion_line.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "FrozenDocumentIndicator")]
    pub frozen_document_indicator: Option<udt::Indicator>,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: cct::Code,
    #[serde(default, rename = "ForecastPeriod")]
    pub forecast_period: Option<crate::Period>,
    #[serde(default, rename = "SalesItem")]
    pub sales_item: Option<crate::SalesItem>,
}
