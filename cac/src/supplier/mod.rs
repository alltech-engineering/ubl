use serde::{Deserialize, Serialize};


include!("party.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct SupplierConsumption {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "UtilitySupplierParty")]
    pub utility_supplier_party: Option<crate::Party>,
    #[serde(default, rename = "UtilityCustomerParty")]
    pub utility_customer_party: Option<crate::Party>,
    #[serde(rename = "Consumption")]
    pub consumption: crate::Consumption,
    #[serde(default, rename = "Contract")]
    pub contract: Option<crate::Contract>,
    #[serde(default, rename = "ConsumptionLine")]
    pub consumption_line: Vec<crate::ConsumptionLine>,
}
