#[derive(Debug, Deserialize, Serialize)]
pub struct SupplierConsumption {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "UtilitySupplierParty")]
    pub utility_supplier_party: Option<Party>,
    #[serde(default, rename = "UtilityCustomerParty")]
    pub utility_customer_party: Option<Party>,
    #[serde(rename = "Consumption")]
    pub consumption: Consumption,
    #[serde(default, rename = "Contract")]
    pub contract: Option<Contract>,
    #[serde(default, rename = "ConsumptionLine")]
    pub consumption_line: Vec<ConsumptionLine>,
}
