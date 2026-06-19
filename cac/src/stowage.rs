#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a location on board a means of transport where specified goods or transport
/// equipment have been stowed or are to be stowed.
///
/// UBL Dictionary Entry Name: `Stowage. Details`
///
/// Generated from XSD type `StowageType`.
pub struct Stowage {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the location.
    #[serde(default, rename = "LocationID")]
    pub location_id: Option<cct::Identifier>,
/// Text describing the location.
    #[serde(default, rename = "Location")]
    pub location: Vec<cct::Text>,
/// A measurable dimension (length, mass, weight, or volume) of this stowage.
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<Dimension>,
}
