#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an ordered shipment.
///
/// UBL Dictionary Entry Name: `Ordered Shipment. Details`
///
/// Generated from XSD type `OrderedShipmentType`.
pub struct OrderedShipment {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The ordered shipment.
    #[serde(rename = "Shipment")]
    pub shipment: Shipment,
/// A package in this ordered shipment.
    #[serde(default, rename = "Package")]
    pub package: Vec<Package>,
}
