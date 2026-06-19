#[derive(Debug, Deserialize, Serialize)]
pub struct PartyTaxScheme {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "RegistrationName")]
    pub registration_name: Option<cct::Text>,
    #[serde(default, rename = "CompanyID")]
    pub company_id: Option<cct::Identifier>,
    #[serde(default, rename = "TaxLevelCode")]
    pub tax_level_code: Option<cct::Code>,
    #[serde(default, rename = "ExemptionReasonCode")]
    pub exemption_reason_code: Option<cct::Code>,
    #[serde(default, rename = "ExemptionReason")]
    pub exemption_reason: Vec<cct::Text>,
    #[serde(default, rename = "RegistrationAddress")]
    pub registration_address: Option<crate::Address>,
    #[serde(rename = "TaxScheme")]
    pub tax_scheme: crate::TaxScheme,
}
