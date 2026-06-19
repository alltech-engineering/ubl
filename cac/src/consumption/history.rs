#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the measurement of a type of consumption during a particular period, used for
/// the subscriber to get an overview of his consumption
///
/// UBL Dictionary Entry Name: `Consumption History. Details`
///
/// Generated from XSD type `ConsumptionHistoryType`.
pub struct ConsumptionHistory {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A text identifier for the meter measuring the consumption.
    #[serde(default, rename = "MeterNumber")]
    pub meter_number: Option<cct::Text>,
/// The quantity consumed.
    #[serde(rename = "Quantity")]
    pub quantity: cct::Quantity,
/// The monetary amount to be charged for the quantity consumed.
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
/// The consumption level, expressed as a code used explain the consumption quantity, e.g.. diversion
/// from the normal.
    #[serde(default, rename = "ConsumptionLevelCode")]
    pub consumption_level_code: Option<cct::Code>,
/// The consumption level, expressed as text, used explain the consumption quantity, e.g.. diversion
/// from the normal.
    #[serde(default, rename = "ConsumptionLevel")]
    pub consumption_level: Option<cct::Text>,
/// Text describing the consumption itself.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The period during which the consumption took place.
    #[serde(rename = "Period")]
    pub period: crate::Period,
}
