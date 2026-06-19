#[derive(Debug, Deserialize, Serialize)]
pub struct ItemManagementProfile {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "FrozenPeriodDaysNumeric")]
    pub frozen_period_days_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "MinimumInventoryQuantity")]
    pub minimum_inventory_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MultipleOrderQuantity")]
    pub multiple_order_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "OrderIntervalDaysNumeric")]
    pub order_interval_days_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "ReplenishmentOwnerDescription")]
    pub replenishment_owner_description: Vec<cct::Text>,
    #[serde(default, rename = "TargetServicePercent")]
    pub target_service_percent: Option<cct::Numeric>,
    #[serde(default, rename = "TargetInventoryQuantity")]
    pub target_inventory_quantity: Option<cct::Quantity>,
    #[serde(rename = "EffectivePeriod")]
    pub effective_period: crate::Period,
    #[serde(rename = "Item")]
    pub item: Item,
    #[serde(default, rename = "ItemLocationQuantity")]
    pub item_location_quantity: Option<ItemLocationQuantity>,
}
