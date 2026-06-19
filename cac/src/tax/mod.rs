use serde::{Deserialize, Serialize};


include!("category.rs");
include!("scheme.rs");
include!("subtotal.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct TaxTotal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "TaxAmount")]
    pub tax_amount: cct::Amount,
    #[serde(default, rename = "CalculationSequenceNumeric")]
    pub calculation_sequence_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "RoundingAmount")]
    pub rounding_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxEvidenceIndicator")]
    pub tax_evidence_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "TaxIncludedIndicator")]
    pub tax_included_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "TaxSubtotal")]
    pub tax_subtotal: Vec<TaxSubtotal>,
}
