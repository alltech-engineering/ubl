#[derive(Debug, Deserialize, Serialize)]
/// A numeric value determined by measuring an object using a specified unit of measure.
///
/// UBL Dictionary Entry Name: `Measure. Type`
///
/// Generated from XSD type `MeasureType`.
pub struct Measure {
/// The type of unit of measure.
    #[serde(default, rename = "@unitCode")]
    pub unit_code: Option<String>,
/// (Deprecated) The version of the measure unit code list.
    #[serde(default, rename = "@unitCodeListVersionID")]
    pub unit_code_list_version_id: Option<String>,
    #[serde(rename = "$text")]
    pub content: f64,
}
