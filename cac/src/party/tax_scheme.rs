#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a taxation scheme applying to a party.
///
/// UBL Dictionary Entry Name: `Party Tax Scheme. Details`
///
/// Generated from XSD type `PartyTaxSchemeType`.
pub struct PartyTaxScheme {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The name of the party as registered with the relevant fiscal authority.
    #[serde(default, rename = "RegistrationName")]
    pub registration_name: Option<cct::Text>,
/// An identifier for the party assigned for tax purposes by the taxation authority.
    #[serde(default, rename = "CompanyID")]
    pub company_id: Option<cct::Identifier>,
/// A code signifying the tax level applicable to the party within this taxation scheme.
    #[serde(default, rename = "TaxLevelCode")]
    pub tax_level_code: Option<cct::Code>,
/// A reason for the party's exemption from tax, expressed as a code.
    #[serde(default, rename = "ExemptionReasonCode")]
    pub exemption_reason_code: Option<cct::Code>,
/// A reason for the party's exemption from tax, expressed as text.
    #[serde(default, rename = "ExemptionReason")]
    pub exemption_reason: Vec<cct::Text>,
/// The address of the party as registered for tax purposes.
    #[serde(default, rename = "RegistrationAddress")]
    pub registration_address: Option<crate::Address>,
/// The taxation scheme applicable to the party.
    #[serde(rename = "TaxScheme")]
    pub tax_scheme: crate::TaxScheme,
}
