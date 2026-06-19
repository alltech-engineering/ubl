#[derive(Debug, Deserialize, Serialize)]
/// A class defining details about a train wagon used as a means of transport.
///
/// UBL Dictionary Entry Name: `Rail Transport. Details`
///
/// Generated from XSD type `RailTransportType`.
pub struct RailTransport {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the train used as the means of transport.
    #[serde(rename = "TrainID")]
    pub train_id: cct::Identifier,
/// An identifier for the rail car on the train used as the means of transport.
    #[serde(default, rename = "RailCarID")]
    pub rail_car_id: Option<cct::Identifier>,
}
