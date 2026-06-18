#[derive(Debug, Deserialize, Serialize)]
pub struct Fee {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "FeeTypeCode")]
    pub fee_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "FeeAmount")]
    pub fee_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "FeeDescription")]
    pub fee_description: Vec<super::cct::TextType>,
}
