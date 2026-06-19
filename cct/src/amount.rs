#[derive(Debug, Deserialize, Serialize)]
pub struct Amount {
    #[serde(default, rename = "@currencyID")]
    pub currency_id: Option<String>,
    #[serde(default, rename = "@currencyCodeListVersionID")]
    pub currency_code_list_version_id: Option<String>,
    #[serde(rename = "$text")]
    pub content: f64,
}
