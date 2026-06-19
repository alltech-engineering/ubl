#[derive(Debug, Deserialize, Serialize)]
pub struct Duty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Amount")]
    pub amount: cct::Amount,
    #[serde(default, rename = "Duty")]
    pub duty: Option<cct::Text>,
    #[serde(default, rename = "DutyCode")]
    pub duty_code: Option<cct::Code>,
    #[serde(default, rename = "TaxCategory")]
    pub tax_category: Option<TaxCategory>,
}
