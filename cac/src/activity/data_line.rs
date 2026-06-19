#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityDataLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: cct::Code,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<crate::CustomerParty>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<crate::SupplierParty>,
    #[serde(default, rename = "ActivityPeriod")]
    pub activity_period: Option<ActivityPeriod>,
    #[serde(rename = "ActivityOriginLocation")]
    pub activity_origin_location: crate::Location,
    #[serde(default, rename = "ActivityFinalLocation")]
    pub activity_final_location: Option<crate::Location>,
    #[serde(default, rename = "SalesItem")]
    pub sales_item: Vec<crate::SalesItem>,
}
