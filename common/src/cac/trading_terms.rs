#[derive(Debug, Deserialize, Serialize)]
pub struct TradingTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Information")]
    pub information: Vec<super::cct::TextType>,
    #[serde(default, rename = "Reference")]
    pub reference: Option<super::cct::TextType>,
    #[serde(default, rename = "ApplicableAddress")]
    pub applicable_address: Option<Address>,
}
