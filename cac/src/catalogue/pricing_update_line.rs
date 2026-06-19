#[derive(Debug, Deserialize, Serialize)]
pub struct CataloguePricingUpdateLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: Option<crate::CustomerParty>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<crate::SupplierParty>,
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: Vec<crate::ItemLocationQuantity>,
}
