use serde::{Deserialize, Serialize};

include!("exception.rs");
include!("revision_line.rs");
include!("exception_criterion_line.rs");

#[derive(Debug, Deserialize, Serialize)]
/// Detailed information about a particular Forecast Line within a Forecast Document
///
/// UBL Dictionary Entry Name: `Forecast Line. Details`
///
/// Generated from XSD type `ForecastLineType`.
pub struct ForecastLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this forecast line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// An indicator that the status of the forecast is modifiable (true) or not (false).
    #[serde(default, rename = "FrozenDocumentIndicator")]
    pub frozen_document_indicator: Option<udt::Indicator>,
/// A code signifying the type of forecast. Examples: BASE PROMOTIONAL SEASONAL TOTAL
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: cct::Code,
/// The period to which the forecast applies.
    #[serde(default, rename = "ForecastPeriod")]
    pub forecast_period: Option<crate::Period>,
/// Sales information for the item to which this line applies.
    #[serde(default, rename = "SalesItem")]
    pub sales_item: Option<crate::SalesItem>,
}
