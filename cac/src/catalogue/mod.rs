use serde::{Deserialize, Serialize};


include!("request_line.rs");
include!("pricing_update_line.rs");
include!("reference.rs");
include!("line.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line describing the transaction that updates the specification of an item in a
/// catalogue.
///
/// UBL Dictionary Entry Name: `Catalogue Item Specification Update Line. Details`
///
/// Generated from XSD type `CatalogueItemSpecificationUpdateLineType`.
pub struct CatalogueItemSpecificationUpdateLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the line to be updated in a catalogue.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// The customer responsible for the contract associated with the catalogue item.
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: Option<crate::CustomerParty>,
/// The seller/supplier responsible for the contract associated with the catalogue item.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<crate::SupplierParty>,
/// The catalogue item to be updated.
    #[serde(rename = "Item")]
    pub item: crate::Item,
}
