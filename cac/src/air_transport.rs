#[derive(Debug, Deserialize, Serialize)]
/// A class to identify a specific aircraft used for transportation.
///
/// UBL Dictionary Entry Name: `Air Transport. Details`
///
/// Generated from XSD type `AirTransportType`.
pub struct AirTransport {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifer for a specific aircraft.
    #[serde(rename = "AircraftID")]
    pub aircraft_id: cct::Identifier,
}
