// UBL Monetary totals — legal and monetary totals for invoices.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalTotal {
    pub line_extension_amount: LineExtensionAmount,
    pub tax_exclusive_amount: Option<TaxExclusiveAmount>,
    pub tax_inclusive_amount: Option<TaxInclusiveAmount>,
    pub allowance_total_amount: Option<AllowanceTotalAmount>,
    pub charge_total_amount: Option<ChargeTotalAmount>,
    pub prepaid_amount: Option<PrepaidAmount>,
    pub payable_rounding_amount: Option<PayableRoundingAmount>,
    pub payable_amount: PayableAmount,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonetaryTotal {
    pub line_extension_amount: Option<LineExtensionAmount>,
    pub tax_exclusive_amount: Option<TaxExclusiveAmount>,
    pub tax_inclusive_amount: Option<TaxInclusiveAmount>,
    pub allowance_total_amount: Option<AllowanceTotalAmount>,
    pub charge_total_amount: Option<ChargeTotalAmount>,
    pub prepaid_amount: Option<PrepaidAmount>,
    pub payable_rounding_amount: Option<PayableRoundingAmount>,
    pub payable_amount: PayableAmount,
    pub payable_alternative_amount: Option<PayableAmount>,
    pub allowance_total_tax_inclusive_amount: Option<AllowanceTotalTaxInclusiveAmount>,
    pub charge_total_tax_inclusive_amount: Option<ChargeTotalTaxInclusiveAmount>,
    pub withholding_tax_total_amount: Option<WithholdingTaxTotalAmount>,
}
