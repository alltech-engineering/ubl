#[derive(Debug, Deserialize, Serialize)]
pub struct Duty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Amount")]
    pub amount: super::cct::AmountType,
    #[serde(default, rename = "Duty")]
    pub duty: Option<super::cct::TextType>,
    #[serde(default, rename = "DutyCode")]
    pub duty_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TaxCategory")]
    pub tax_category: Option<TaxCategory>,
}
