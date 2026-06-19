#[derive(Debug, Deserialize, Serialize)]
pub struct TelecommunicationsSupplyLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: cct::Text,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Vec<crate::ExchangeRate>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
    #[serde(default, rename = "TelecommunicationsService")]
    pub telecommunications_service: Vec<TelecommunicationsService>,
}
