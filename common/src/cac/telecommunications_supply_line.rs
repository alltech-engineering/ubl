#[derive(Debug, Deserialize, Serialize)]
pub struct TelecommunicationsSupplyLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: super::cct::TextType,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Vec<ExchangeRate>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<TaxTotal>,
    #[serde(default, rename = "TelecommunicationsService")]
    pub telecommunications_service: Vec<TelecommunicationsService>,
}
