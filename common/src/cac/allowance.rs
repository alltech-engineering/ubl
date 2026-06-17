// UBL Allowance and Charge aggregate.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

use crate::cac::tax::TaxCategory;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowanceCharge {
    #[serde(default)]
    pub id: Option<ID>,
    pub charge_indicator: ChargeIndicator,
    #[serde(default)]
    pub allowance_charge_reason_code: Option<AllowanceChargeReasonCode>,
    #[serde(default)]
    pub allowance_charge_reason: Vec<AllowanceChargeReason>,
    #[serde(default)]
    pub multiplier_factor_numeric: Option<Numeric>,
    #[serde(default)]
    pub prepaid_indicator: Option<Indicator>,
    #[serde(default)]
    pub sequence_numeric: Option<Numeric>,
    pub amount: Amount,
    #[serde(default)]
    pub base_amount: Option<BaseAmount>,
    #[serde(default)]
    pub tax_inclusive_amount: Option<TaxInclusiveAmount>,
    #[serde(default)]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(default)]
    pub accounting_cost: Option<AccountingCost>,
    #[serde(default)]
    pub per_unit_amount: Option<PerUnitAmount>,
    #[serde(default)]
    pub tax_category: Vec<TaxCategory>,
    #[serde(default)]
    pub tax_total: Option<TaxTotal>,
    #[serde(default)]
    pub payment_means: Vec<PaymentMeans>,
}

use crate::cac::payment::PaymentMeans;
use crate::cac::tax::TaxTotal;
