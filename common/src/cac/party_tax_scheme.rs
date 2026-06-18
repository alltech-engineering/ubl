#[derive(Debug, Deserialize, Serialize)]
pub struct PartyTaxScheme {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "RegistrationName")]
    pub registration_name: Option<super::cct::TextType>,
    #[serde(default, rename = "CompanyID")]
    pub company_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TaxLevelCode")]
    pub tax_level_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ExemptionReasonCode")]
    pub exemption_reason_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ExemptionReason")]
    pub exemption_reason: Vec<super::cct::TextType>,
    #[serde(default, rename = "RegistrationAddress")]
    pub registration_address: Option<Address>,
    #[serde(rename = "TaxScheme")]
    pub tax_scheme: TaxScheme,
}
