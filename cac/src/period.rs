#[derive(Debug, Deserialize, Serialize)]
pub struct Period {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "StartDate")]
    pub start_date: Option<udt::DateTime>,
    #[serde(default, rename = "StartTime")]
    pub start_time: Option<udt::DateTime>,
    #[serde(default, rename = "EndDate")]
    pub end_date: Option<udt::DateTime>,
    #[serde(default, rename = "EndTime")]
    pub end_time: Option<udt::DateTime>,
    #[serde(default, rename = "DurationMeasure")]
    pub duration_measure: Option<cct::Measure>,
    #[serde(default, rename = "DescriptionCode")]
    pub description_code: Vec<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
