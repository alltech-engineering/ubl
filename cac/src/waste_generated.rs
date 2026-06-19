#[derive(Debug, Deserialize, Serialize)]
pub struct WasteGenerated {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "WasteTypeCode")]
    pub waste_type_code: Option<cct::Code>,
    #[serde(default, rename = "WasteTypeDescription")]
    pub waste_type_description: Vec<cct::Text>,
    #[serde(rename = "WasteMeasure")]
    pub waste_measure: cct::Measure,
    #[serde(default, rename = "MeasurementPeriod")]
    pub measurement_period: Option<Period>,
}
