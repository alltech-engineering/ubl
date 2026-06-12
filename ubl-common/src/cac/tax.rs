// UBL Tax aggregates — tax schemes, categories, subtotals, and totals.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

use crate::cac::address::Address;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxScheme {
    pub id: Option<TaxSchemeID>,
    pub tax_type_code: Option<TaxTypeCode>,
    pub name: Option<Name>,
    pub jurisdiction_region_address: Vec<Address>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxCategory {
    pub id: Option<ID>,
    pub name: Option<Name>,
    pub percent: Option<Percent>,
    pub base_unit_measure: Option<BaseUnitMeasure>,
    pub per_unit_amount: Option<PerUnitAmount>,
    pub tax_exemption_reason_code: Option<TaxExemptionReasonCode>,
    pub tax_exemption_reason: Vec<TaxExemptionReason>,
    pub tier_range: Option<TierRange>,
    pub tier_rate_percent: Option<TierRatePercent>,
    pub tax_scheme: TaxScheme,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxSubtotal {
    pub taxable_amount: Option<TaxableAmount>,
    pub tax_amount: TaxAmount,
    pub calculation_sequence_numeric: Option<CalculationSequenceNumeric>,
    pub transaction_currency_tax_amount: Option<TransactionCurrencyTaxAmount>,
    pub percent: Option<Percent>,
    pub base_unit_measure: Option<BaseUnitMeasure>,
    pub per_unit_amount: Option<PerUnitAmount>,
    pub tier_range: Option<TierRange>,
    pub tier_rate_percent: Option<TierRatePercent>,
    pub tax_category: TaxCategory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxTotal {
    pub tax_amount: TaxAmount,
    pub rounding_amount: Option<RoundingAmount>,
    pub tax_evidence_indicator: Option<TaxEvidenceIndicator>,
    pub tax_included_indicator: Option<TaxIncludedIndicator>,
    pub tax_subtotal: Vec<TaxSubtotal>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TierRange {
    pub id: Option<ID>,
    pub description: Option<Description>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PricingReference {
    pub original_item_location_quantity: Option<Quantity>,
    pub alternative_condition_price: Vec<Price>,
}

use crate::cac::price::Price;
