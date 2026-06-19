#[derive(Debug, Deserialize, Serialize)]
pub struct Prize {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "RankCode")]
    pub rank_code: cct::Code,
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: Option<cct::Amount>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
