#[derive(Debug, Deserialize, Serialize)]
pub struct CreditAccount {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "AccountID")]
    pub account_id: cct::Identifier,
}
