#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsItemContainer {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
    #[serde(default, rename = "TransportEquipment")]
    pub transport_equipment: Vec<crate::TransportEquipment>,
}
