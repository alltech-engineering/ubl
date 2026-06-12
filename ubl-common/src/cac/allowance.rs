// UBL Allowance and Charge aggregate.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

use crate::cac::tax::TaxCategory;
use crate::cac::period::Period;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowanceCharge {
    pub id: Option<ID>,
    pub charge_indicator: ChargeIndicator,
    pub allowance_charge_reason_code: Option<AllowanceChargeReasonCode>,
    pub allowance_charge_reason: Vec<AllowanceChargeReason>,
    pub multiplier_factor_numeric: Option<Numeric>,
    pub prepaid_indicator: Option<Indicator>,
    pub sequence_numeric: Option<Numeric>,
    pub amount: Amount,
    pub base_amount: Option<BaseAmount>,
    pub accounting_cost_code: Option<AccountingCostCode>,
    pub accounting_cost: Option<AccountingCost>,
    pub per_unit_amount: Option<PerUnitAmount>,
    pub tax_category: Vec<TaxCategory>,
    pub tax_total: Option<TaxTotal>,
    pub payment_means: Vec<PaymentMeans>,
}

use crate::cac::tax::TaxTotal;
use crate::cac::payment::PaymentMeans;
