#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a planned effect of a retail event (e.g., a promotion or a change in inventory
/// policy) upon supply or demand.
///
/// UBL Dictionary Entry Name: `Retail Planned Impact. Details`
///
/// Generated from XSD type `RetailPlannedImpactType`.
pub struct RetailPlannedImpact {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// Estimated monetary value of the planned event as an impact
    #[serde(rename = "Amount")]
    pub amount: cct::Amount,
/// It will have impact on either Sales forecast or Order Forecast
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: cct::Code,
/// A code signifying the type of forecast. Examples of values are: BASE PROMOTIONAL SEASONAL TOTAL
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: cct::Code,
/// The period to which this impact applies.
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
}
