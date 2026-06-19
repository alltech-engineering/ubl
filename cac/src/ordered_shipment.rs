#[derive(Debug, Deserialize, Serialize)]
pub struct OrderedShipment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "Shipment")]
    pub shipment: Shipment,
    #[serde(default, rename = "Package")]
    pub package: Vec<Package>,
}
