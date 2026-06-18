#[derive(Debug, Deserialize, Serialize)]
pub struct AirTransport {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "AircraftID")]
    pub aircraft_id: super::cct::IdentifierType,
}
