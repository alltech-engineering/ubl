#[derive(Debug, Deserialize, Serialize)]
/// A class to associate a time period and locations (activity data) with an item for inventory planning
/// purposes.
///
/// UBL Dictionary Entry Name: `Activity Data Line. Details`
///
/// Generated from XSD type `ActivityDataLineType`.
pub struct ActivityDataLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this activity data line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A code signifying the type of supply chain activity.
    #[serde(rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: cct::Code,
/// The buyer of the item.
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<crate::CustomerParty>,
/// The seller of the item.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<crate::SupplierParty>,
/// The period during which the activity is realized.
    #[serde(default, rename = "ActivityPeriod")]
    pub activity_period: Option<ActivityPeriod>,
/// Either the location where the movement of goods is observed or the location from which the goods are
/// moved.
    #[serde(rename = "ActivityOriginLocation")]
    pub activity_origin_location: crate::Location,
/// The location to which the goods are moved.
    #[serde(default, rename = "ActivityFinalLocation")]
    pub activity_final_location: Option<crate::Location>,
/// Sales information for an item to which this line applies.
    #[serde(default, rename = "SalesItem")]
    pub sales_item: Vec<crate::SalesItem>,
}
