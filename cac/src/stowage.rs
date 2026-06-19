#[derive(Debug, Deserialize, Serialize)]
pub struct Stowage {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "LocationID")]
    pub location_id: Option<cct::Identifier>,
    #[serde(default, rename = "Location")]
    pub location: Vec<cct::Text>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<Dimension>,
}
