#[derive(Debug, Deserialize, Serialize)]
pub struct TradingTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Information")]
    pub information: Vec<cct::Text>,
    #[serde(default, rename = "Reference")]
    pub reference: Option<cct::Text>,
    #[serde(default, rename = "ApplicableAddress")]
    pub applicable_address: Option<Address>,
}
