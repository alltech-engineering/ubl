#[derive(Debug, Deserialize, Serialize)]
pub struct SanitaryMeasure {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "SanitaryMeasureTypeCode")]
    pub sanitary_measure_type_code: super::cct::CodeType,
    #[serde(default, rename = "ApplicationDate")]
    pub application_date: Option<super::udt::DateTimeType>,
}
