#[derive(Debug, Deserialize, Serialize)]
/// A class for identifying a vehicle used for road transport.
///
/// UBL Dictionary Entry Name: `Road Transport. Details`
///
/// Generated from XSD type `RoadTransportType`.
pub struct RoadTransport {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The license plate identifier of this vehicle.
    #[serde(rename = "LicensePlateID")]
    pub license_plate_id: cct::Identifier,
/// The license plate identifier of a trailer pulled by this vehicle.
    #[serde(default, rename = "TrailerLicensePlateID")]
    pub trailer_license_plate_id: Option<cct::Identifier>,
}
