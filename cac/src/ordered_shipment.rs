#[derive(Debug, Deserialize, Serialize)]
pub struct OrderedShipment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Shipment")]
    pub shipment: Shipment,
    #[serde(default, rename = "Package")]
    pub package: Vec<Package>,
}
