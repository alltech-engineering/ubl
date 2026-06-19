#[derive(Debug, Deserialize, Serialize)]
pub struct AirTransport {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "AircraftID")]
    pub aircraft_id: cct::Identifier,
}
