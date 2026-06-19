use serde::{Deserialize, Serialize};


include!("category.rs");
include!("scheme.rs");
include!("subtotal.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the total tax for a particular taxation scheme.
///
/// UBL Dictionary Entry Name: `Tax Total. Details`
///
/// Generated from XSD type `TaxTotalType`.
pub struct TaxTotal {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The total tax amount for a particular taxation scheme, e.g., VAT; the sum of the tax subtotals for
/// each tax category within the taxation scheme.
    #[serde(rename = "TaxAmount")]
    pub tax_amount: cct::Amount,
/// The number of this tax total in the sequence of tax totals corresponding to the order in which
/// multiple taxes are applied. If all taxes are applied to the same taxable amount (i.e., their order
/// of application is inconsequential), then CalculationSequenceNumeric is 1 for all tax totals applied
/// to a given amount.
    #[serde(default, rename = "CalculationSequenceNumeric")]
    pub calculation_sequence_numeric: Option<cct::Numeric>,
/// The rounding amount (positive or negative) added to the calculated tax total to produce the rounded
/// TaxAmount.
    #[serde(default, rename = "RoundingAmount")]
    pub rounding_amount: Option<cct::Amount>,
/// An indicator that this total is recognized as legal evidence for taxation purposes (true) or not
/// (false).
    #[serde(default, rename = "TaxEvidenceIndicator")]
    pub tax_evidence_indicator: Option<udt::Indicator>,
/// An indicator that tax is included in the calculation (true) or not (false).
    #[serde(default, rename = "TaxIncludedIndicator")]
    pub tax_included_indicator: Option<udt::Indicator>,
/// One of the subtotals the sum of which equals the total tax amount for a particular taxation scheme.
    #[serde(default, rename = "TaxSubtotal")]
    pub tax_subtotal: Vec<TaxSubtotal>,
}
