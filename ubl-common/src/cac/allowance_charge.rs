// AllowanceCharge — UBL CAC aggregate (Tier 1 stub)
use crate::cbc::*;

#[derive(Debug, Clone, Partialserde::Serialize, serde::Deserialize)]
pub struct AllowanceCharge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_indicator: Option<ChargeIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowance_charge_reason_code: Option<AllowanceChargeReasonCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier_factor_numeric: Option<MultiplierFactorNumeric>,
}
