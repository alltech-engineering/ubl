#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line describing a pricing update to a catalogue line.
///
/// UBL Dictionary Entry Name: `Catalogue Pricing Update Line. Details`
///
/// Generated from XSD type `CataloguePricingUpdateLineType`.
pub struct CataloguePricingUpdateLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the catalogue line to be updated.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// The customer responsible for the contract associated with the catalogue line being updated.
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: Option<crate::CustomerParty>,
/// The seller/supplier responsible for the contract associated with the catalogue line being updated.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<crate::SupplierParty>,
/// Updated properties of the item in this catalogue line that are dependent on location and quantity.
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: Vec<crate::ItemLocationQuantity>,
}
