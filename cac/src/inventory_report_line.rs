#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in an Inventory Report.
///
/// UBL Dictionary Entry Name: `Inventory Report Line. Details`
///
/// Generated from XSD type `InventoryReportLineType`.
pub struct InventoryReportLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this inventory report line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The quantity of the item reported that is currently in stock.
    #[serde(rename = "Quantity")]
    pub quantity: cct::Quantity,
/// The value of the quantity of the item reported that is currently in stock.
    #[serde(default, rename = "InventoryValueAmount")]
    pub inventory_value_amount: Option<cct::Amount>,
/// The date from which the goods will be available. If not present, the goods are available now.
    #[serde(default, rename = "AvailabilityDate")]
    pub availability_date: Option<udt::DateTime>,
/// A code signifying the item's level of availability.
    #[serde(default, rename = "AvailabilityStatusCode")]
    pub availability_status_code: Option<cct::Code>,
/// The item associated with this inventory report line.
    #[serde(rename = "Item")]
    pub item: Item,
/// The location of the reported quantity of goods.
    #[serde(default, rename = "InventoryLocation")]
    pub inventory_location: Option<Location>,
}
