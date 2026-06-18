#[derive(Debug, Deserialize, Serialize)]
pub struct WasteGenerated {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "WasteTypeCode")]
    pub waste_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "WasteTypeDescription")]
    pub waste_type_description: Vec<super::cct::TextType>,
    #[serde(rename = "WasteMeasure")]
    pub waste_measure: super::cct::MeasureType,
    #[serde(default, rename = "MeasurementPeriod")]
    pub measurement_period: Option<Period>,
}
