#[derive(Debug, Deserialize, Serialize)]
pub struct CreditAccount {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "AccountID")]
    pub account_id: cct::Identifier,
}
