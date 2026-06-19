use serde::{Deserialize, Serialize};


include!("request_line.rs");
include!("pricing_update_line.rs");
include!("reference.rs");
include!("line.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueItemSpecificationUpdateLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: Option<crate::CustomerParty>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<crate::SupplierParty>,
    #[serde(rename = "Item")]
    pub item: crate::Item,
}
