#[derive(Debug, Deserialize, Serialize)]
/// A class to define a management profile for an item.
///
/// UBL Dictionary Entry Name: `Item Management Profile. Details`
///
/// Generated from XSD type `ItemManagementProfileType`.
pub struct ItemManagementProfile {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The number of days in the future that an order forecast quantity automatically becomes a confirmed
/// order for a product.
    #[serde(default, rename = "FrozenPeriodDaysNumeric")]
    pub frozen_period_days_numeric: Option<cct::Numeric>,
/// The quantity of the item that will trigger a replenishment order to avoid depleting the safety
/// stock.
    #[serde(default, rename = "MinimumInventoryQuantity")]
    pub minimum_inventory_quantity: Option<cct::Quantity>,
/// The order quantity multiples in which the product may be ordered.
    #[serde(default, rename = "MultipleOrderQuantity")]
    pub multiple_order_quantity: Option<cct::Quantity>,
/// The number of days between regular replenishment orders for the product.
    #[serde(default, rename = "OrderIntervalDaysNumeric")]
    pub order_interval_days_numeric: Option<cct::Numeric>,
/// The trading partner maintaining this item management profile.
    #[serde(default, rename = "ReplenishmentOwnerDescription")]
    pub replenishment_owner_description: Vec<cct::Text>,
/// The Unit Service Level the trading partners expect to be maintained, expressed as a percentage.
/// Unite Service Level (USL) is a term used in Inventory Management, which is sometimes known as "fill
/// rate", counts the average number of units short expressed as the percentage of the order quantity.
    #[serde(default, rename = "TargetServicePercent")]
    pub target_service_percent: Option<cct::Numeric>,
/// The target inventory quantity.
    #[serde(default, rename = "TargetInventoryQuantity")]
    pub target_inventory_quantity: Option<cct::Quantity>,
/// The period during which this profile is effective.
    #[serde(rename = "EffectivePeriod")]
    pub effective_period: crate::Period,
/// The item associated with this item management profile.
    #[serde(rename = "Item")]
    pub item: Item,
/// A set of location-specific properties (e.g., price and quantity) associated with the item.
    #[serde(default, rename = "ItemLocationQuantity")]
    pub item_location_quantity: Option<ItemLocationQuantity>,
}
