#[derive(Debug, Deserialize, Serialize)]
/// A class to describe subcontract terms for a tendering process.
///
/// UBL Dictionary Entry Name: `Subcontract Terms. Details`
///
/// Generated from XSD type `SubcontractTermsType`.
pub struct SubcontractTerms {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The precise percentage allowed to be subcontracted.
    #[serde(default, rename = "Rate")]
    pub rate: Option<cct::Numeric>,
/// (Updated definition) An indicator that the subcontract price is unknown (true) or not (false).
    #[serde(default, rename = "UnknownPriceIndicator")]
    pub unknown_price_indicator: Option<udt::Indicator>,
/// Text describing the subcontract terms.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The monetary amount assigned to the subcontracted task.
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
/// A code specifying the conditions for subcontracting.
    #[serde(default, rename = "SubcontractingConditionsCode")]
    pub subcontracting_conditions_code: Option<cct::Code>,
/// The maximum percentage allowed to be subcontracted.
    #[serde(default, rename = "MaximumPercent")]
    pub maximum_percent: Option<cct::Numeric>,
/// The minimum percentage allowed to be subcontracted.
    #[serde(default, rename = "MinimumPercent")]
    pub minimum_percent: Option<cct::Numeric>,
}
