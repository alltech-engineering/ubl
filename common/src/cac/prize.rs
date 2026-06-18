#[derive(Debug, Deserialize, Serialize)]
pub struct Prize {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "RankCode")]
    pub rank_code: super::cct::CodeType,
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
}
