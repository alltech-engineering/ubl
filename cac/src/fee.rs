#[derive(Debug, Deserialize, Serialize)]
pub struct Fee {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "FeeTypeCode")]
    pub fee_type_code: Option<cct::Code>,
    #[serde(default, rename = "FeeAmount")]
    pub fee_amount: Option<cct::Amount>,
    #[serde(default, rename = "FeeDescription")]
    pub fee_description: Vec<cct::Text>,
}
