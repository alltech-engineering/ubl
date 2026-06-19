#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Performance History.
///
/// UBL Dictionary Entry Name: `Performance Data Line. Details`
///
/// Generated from XSD type `PerformanceDataLineType`.
pub struct PerformanceDataLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this performance data line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The value of the reported attribute.
    #[serde(rename = "PerformanceValueQuantity")]
    pub performance_value_quantity: cct::Quantity,
/// A code signifying the measure of performance applicable to the reported attribute.
    #[serde(rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: cct::Code,
/// The period to which this performance data line applies.
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
/// The item whose performance is reported in this data line.
    #[serde(default, rename = "Item")]
    pub item: Option<Item>,
}
