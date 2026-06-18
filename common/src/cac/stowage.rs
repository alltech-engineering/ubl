#[derive(Debug, Deserialize, Serialize)]
pub struct Stowage {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "LocationID")]
    pub location_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Location")]
    pub location: Vec<super::cct::TextType>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<Dimension>,
}
