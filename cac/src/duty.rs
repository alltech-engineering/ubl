#[derive(Debug, Deserialize, Serialize)]
/// The charging rate used for both call charging and time dependent charging
///
/// UBL Dictionary Entry Name: `Duty. Details`
///
/// Generated from XSD type `DutyType`.
pub struct Duty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The amount of this duty.
    #[serde(rename = "Amount")]
    pub amount: cct::Amount,
/// Text describing this duty.
    #[serde(default, rename = "Duty")]
    pub duty: Option<cct::Text>,
/// The type of this charge rate, expressed as a code.
    #[serde(default, rename = "DutyCode")]
    pub duty_code: Option<cct::Code>,
/// The tax category applicable to this duty.
    #[serde(default, rename = "TaxCategory")]
    pub tax_category: Option<TaxCategory>,
}
