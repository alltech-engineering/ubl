#[derive(Debug, Deserialize, Serialize)]
/// A class defining how goods items are split across transport equipment.
///
/// UBL Dictionary Entry Name: `Goods Item Container. Details`
///
/// Generated from XSD type `GoodsItemContainerType`.
pub struct GoodsItemContainer {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this goods item container.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// The number of goods items loaded into or onto one piece of transport equipment as a total
/// consignment or part of a consignment.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// A piece of transport equipment used to contain a single goods item.
    #[serde(default, rename = "TransportEquipment")]
    pub transport_equipment: Vec<crate::TransportEquipment>,
}
