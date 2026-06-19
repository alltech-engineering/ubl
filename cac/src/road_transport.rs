#[derive(Debug, Deserialize, Serialize)]
pub struct RoadTransport {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "LicensePlateID")]
    pub license_plate_id: cct::Identifier,
    #[serde(default, rename = "TrailerLicensePlateID")]
    pub trailer_license_plate_id: Option<cct::Identifier>,
}
