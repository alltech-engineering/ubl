#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a period of time.
///
/// UBL Dictionary Entry Name: `Period. Details`
///
/// Generated from XSD type `PeriodType`.
pub struct Period {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The date on which this period begins.
    #[serde(default, rename = "StartDate")]
    pub start_date: Option<udt::DateTime>,
/// The time at which this period begins.
    #[serde(default, rename = "StartTime")]
    pub start_time: Option<udt::DateTime>,
/// The date on which this period ends.
    #[serde(default, rename = "EndDate")]
    pub end_date: Option<udt::DateTime>,
/// The time at which this period ends.
    #[serde(default, rename = "EndTime")]
    pub end_time: Option<udt::DateTime>,
/// The duration of this period, expressed as an ISO 8601 code.
    #[serde(default, rename = "DurationMeasure")]
    pub duration_measure: Option<cct::Measure>,
/// (Endorsed cardinality: 0..1) A description of this period, expressed as a code.
    #[serde(default, rename = "DescriptionCode")]
    pub description_code: Vec<cct::Code>,
/// A description of this period, expressed as text.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
