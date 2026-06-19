#[derive(Debug, Deserialize, Serialize)]
/// A class for describing the terms of a trade agreement.
///
/// UBL Dictionary Entry Name: `Trading Terms. Details`
///
/// Generated from XSD type `TradingTermsType`.
pub struct TradingTerms {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// Text describing the terms of a trade agreement.
    #[serde(default, rename = "Information")]
    pub information: Vec<cct::Text>,
/// A reference quoting the basis of the terms
    #[serde(default, rename = "Reference")]
    pub reference: Option<cct::Text>,
/// The address at which these trading terms apply.
    #[serde(default, rename = "ApplicableAddress")]
    pub applicable_address: Option<Address>,
}
