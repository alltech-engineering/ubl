#[derive(Debug, Deserialize, Serialize)]
/// A class to define an average consumption as a monetary amount.
///
/// UBL Dictionary Entry Name: `Consumption Average. Details`
///
/// Generated from XSD type `ConsumptionAverageType`.
pub struct ConsumptionAverage {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The average monetary amount of the consumption.
    #[serde(default, rename = "AverageAmount")]
    pub average_amount: Option<cct::Amount>,
/// A description of the average consumed.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
