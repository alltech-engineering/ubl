// UBL Monetary totals — legal and monetary totals for invoices.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_total_roundtrip() {
        use rust_decimal::Decimal;
        let lt = LegalTotal {
            line_extension_amount: LineExtensionAmount::new(Decimal::new(10000, 2), "ZAR"),
            tax_exclusive_amount: Some(TaxExclusiveAmount::new(Decimal::new(10000, 2), "ZAR")),
            tax_inclusive_amount: Some(TaxInclusiveAmount::new(Decimal::new(11500, 2), "ZAR")),
            allowance_total_amount: None,
            charge_total_amount: None,
            prepaid_amount: None,
            payable_rounding_amount: None,
            payable_amount: PayableAmount::new(Decimal::new(11500, 2), "ZAR"),
        };
        let json = serde_json::to_string(&lt).unwrap();
        let lt2: LegalTotal = serde_json::from_str(&json).unwrap();
        assert_eq!(*lt.payable_amount.value(), *lt2.payable_amount.value());
    }
}
