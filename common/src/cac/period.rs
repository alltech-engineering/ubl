#[derive(Debug, Deserialize, Serialize)]
pub struct Period {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "StartDate")]
    pub start_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "StartTime")]
    pub start_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EndDate")]
    pub end_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EndTime")]
    pub end_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "DurationMeasure")]
    pub duration_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "DescriptionCode")]
    pub description_code: Vec<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
}
