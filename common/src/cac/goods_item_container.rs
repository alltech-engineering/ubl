#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsItemContainer {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "TransportEquipment")]
    pub transport_equipment: Vec<TransportEquipment>,
}
