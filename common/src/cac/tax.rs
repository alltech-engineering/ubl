// UBL Tax aggregates — tax schemes, categories, subtotals, and totals.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

use crate::cac::address::Address;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxScheme {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub tax_type_code: Option<TaxTypeCode>,
    #[serde(default)]
    pub currency_code: Option<CurrencyCode>,
    #[serde(default)]
    pub name: Option<Name>,
    #[serde(default)]
    pub jurisdiction_region_address: Vec<Address>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxCategory {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub supply_type_code: Option<SupplyTypeCode>,
    #[serde(default)]
    pub name: Option<Name>,
    #[serde(default)]
    pub percent: Option<Percent>,
    #[serde(default)]
    pub base_unit_measure: Option<BaseUnitMeasure>,
    #[serde(default)]
    pub per_unit_amount: Option<PerUnitAmount>,
    #[serde(default)]
    pub tax_exemption_reason_code: Option<TaxExemptionReasonCode>,
    #[serde(default)]
    pub tax_exemption_reason: Vec<TaxExemptionReason>,
    #[serde(default)]
    pub tier_range: Option<TierRange>,
    #[serde(default)]
    pub tier_rate_percent: Option<TierRatePercent>,
    pub tax_scheme: TaxScheme,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxSubtotal {
    #[serde(default)]
    pub taxable_amount: Option<TaxableAmount>,
    pub tax_amount: TaxAmount,
    #[serde(default)]
    pub calculation_sequence_numeric: Option<CalculationSequenceNumeric>,
    #[serde(default)]
    pub transaction_currency_tax_amount: Option<TransactionCurrencyTaxAmount>,
    #[serde(default)]
    pub percent: Option<Percent>,
    #[serde(default)]
    pub base_unit_measure: Option<BaseUnitMeasure>,
    #[serde(default)]
    pub per_unit_amount: Option<PerUnitAmount>,
    #[serde(default)]
    pub tax_inclusive_amount: Option<TaxInclusiveAmount>,
    #[serde(default)]
    pub tier_range: Option<TierRange>,
    #[serde(default)]
    pub tier_rate_percent: Option<TierRatePercent>,
    pub tax_category: TaxCategory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxTotal {
    pub tax_amount: TaxAmount,
    #[serde(default)]
    pub rounding_amount: Option<RoundingAmount>,
    #[serde(default)]
    pub tax_evidence_indicator: Option<TaxEvidenceIndicator>,
    #[serde(default)]
    pub tax_included_indicator: Option<TaxIncludedIndicator>,
    #[serde(default)]
    pub calculation_sequence_numeric: Option<CalculationSequenceNumeric>,
    #[serde(default)]
    pub tax_subtotal: Vec<TaxSubtotal>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TierRange {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub description: Option<Description>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PricingReference {
    #[serde(default)]
    pub original_item_location_quantity: Option<Quantity>,
    #[serde(default)]
    pub alternative_condition_price: Vec<Price>,
}

use crate::cac::price::Price;

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_tax_scheme() -> TaxScheme {
        TaxScheme {
            id: None,
            tax_type_code: None,
            currency_code: None,
            name: None,
            jurisdiction_region_address: vec![],
        }
    }

    #[test]
    fn test_tax_scheme_roundtrip() {
        let mut ts = empty_tax_scheme();
        ts.id = Some(ID::new("VAT"));
        ts.name = Some(Name::new("VAT"));
        let json = serde_json::to_string(&ts).unwrap();
        let ts2: TaxScheme = serde_json::from_str(&json).unwrap();
        assert_eq!(ts.id.unwrap().value(), ts2.id.unwrap().value());
    }

    #[test]
    fn test_tax_category_roundtrip() {
        let mut ts = empty_tax_scheme();
        ts.id = Some(ID::new("VAT"));
        let tc = TaxCategory {
            id: Some(ID::new("S")),
            supply_type_code: None,
            name: None,
            percent: Some(Percent::new(rust_decimal::Decimal::new(15, 0))),
            base_unit_measure: None,
            per_unit_amount: None,
            tax_exemption_reason_code: None,
            tax_exemption_reason: vec![],
            tier_range: None,
            tier_rate_percent: None,
            tax_scheme: ts,
        };
        let json = serde_json::to_string(&tc).unwrap();
        assert!(json.contains("15"));
        let tc2: TaxCategory = serde_json::from_str(&json).unwrap();
        assert_eq!(tc2.percent.unwrap().0.to_string(), "15");
    }

    #[test]
    fn test_tax_total_roundtrip() {
        use rust_decimal::Decimal;
        let tt = TaxTotal {
            tax_amount: TaxAmount::new(Decimal::new(1500, 2), "ZAR"),
            rounding_amount: None,
            tax_evidence_indicator: None,
            tax_included_indicator: None,
            calculation_sequence_numeric: None,
            tax_subtotal: vec![TaxSubtotal {
                tax_amount: TaxAmount::new(Decimal::new(1500, 2), "ZAR"),
                taxable_amount: Some(TaxableAmount::new(Decimal::new(10000, 2), "ZAR")),
                calculation_sequence_numeric: None,
                transaction_currency_tax_amount: None,
                percent: Some(Percent::new(Decimal::new(15, 0))),
                base_unit_measure: None,
                per_unit_amount: None,
                tier_range: None,
                tier_rate_percent: None,
                tax_inclusive_amount: None,
                tax_category: TaxCategory {
                    id: None,
                    supply_type_code: None,
                    name: None,
                    percent: Some(Percent::new(Decimal::new(15, 0))),
                    base_unit_measure: None,
                    per_unit_amount: None,
                    tax_exemption_reason_code: None,
                    tax_exemption_reason: vec![],
                    tier_range: None,
                    tier_rate_percent: None,
                    tax_scheme: empty_tax_scheme(),
                },
            }],
        };
        let json = serde_json::to_string(&tt).unwrap();
        let tt2: TaxTotal = serde_json::from_str(&json).unwrap();
        assert_eq!(*tt.tax_amount.value(), *tt2.tax_amount.value());
    }
}
