#[derive(Debug, Deserialize, Serialize)]
pub struct CataloguePricingUpdateLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: Option<CustomerParty>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<SupplierParty>,
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: Vec<ItemLocationQuantity>,
}
