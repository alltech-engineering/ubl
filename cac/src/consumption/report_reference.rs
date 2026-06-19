#[derive(Debug, Deserialize, Serialize)]
/// A class to define a reference to an earlier consumption report (e.g., last year's consumption).
///
/// UBL Dictionary Entry Name: `Consumption Report Reference. Details`
///
/// Generated from XSD type `ConsumptionReportReferenceType`.
pub struct ConsumptionReportReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the referenced consumption report.
    #[serde(rename = "ConsumptionReportID")]
    pub consumption_report_id: cct::Identifier,
/// The reported consumption type, expressed as text.
    #[serde(default, rename = "ConsumptionType")]
    pub consumption_type: Option<cct::Text>,
/// The reported consumption type, expressed as a code.
    #[serde(default, rename = "ConsumptionTypeCode")]
    pub consumption_type_code: Option<cct::Code>,
/// The total quantity consumed during the period of the referenced report.
    #[serde(rename = "TotalConsumedQuantity")]
    pub total_consumed_quantity: cct::Quantity,
/// The period of consumption covered by the referenced report.
    #[serde(rename = "Period")]
    pub period: crate::Period,
}
