use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DataLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<crate::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: crate::cct::IdentifierType,
    #[serde(rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: crate::cct::CodeType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<crate::cac::CustomerParty>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<crate::cac::SupplierParty>,
    #[serde(default, rename = "ActivityPeriod")]
    pub activity_period: Option<crate::cac::Period>,
    #[serde(rename = "ActivityOriginLocation")]
    pub activity_origin_location: crate::cac::Location,
    #[serde(default, rename = "ActivityFinalLocation")]
    pub activity_final_location: Option<crate::cac::Location>,
    #[serde(default, rename = "SalesItem")]
    pub sales_item: Vec<crate::cac::SalesItem>,
}
