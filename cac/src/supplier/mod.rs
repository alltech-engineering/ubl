use serde::{Deserialize, Serialize};


include!("party.rs");

#[derive(Debug, Deserialize, Serialize)]
/// The consumption in case the consumption is for one and only one supplier.
///
/// UBL Dictionary Entry Name: `Supplier Consumption. Details`
///
/// Generated from XSD type `SupplierConsumptionType`.
pub struct SupplierConsumption {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The Party who supplies the utility.
    #[serde(default, rename = "UtilitySupplierParty")]
    pub utility_supplier_party: Option<crate::Party>,
/// The Customer Party for this utility.
    #[serde(default, rename = "UtilityCustomerParty")]
    pub utility_customer_party: Option<crate::Party>,
/// The consumption regarding this supplier
    #[serde(rename = "Consumption")]
    pub consumption: crate::Consumption,
/// A contract setting forth conditions regulating the consumption.
    #[serde(default, rename = "Contract")]
    pub contract: Option<crate::Contract>,
/// The consumption of a utility product.
    #[serde(default, rename = "ConsumptionLine")]
    pub consumption_line: Vec<crate::ConsumptionLine>,
}
