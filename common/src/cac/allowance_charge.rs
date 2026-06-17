// AllowanceCharge — UBL CAC aggregate (Tier 1 stub)
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllowanceCharge {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charge_indicator: Option<ChargeIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowance_charge_reason_code: Option<AllowanceChargeReasonCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiplier_factor_numeric: Option<MultiplierFactorNumeric>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_charge_indicator_defaults_to_none() {
        let json = r#"{"amount": {"value": "0", "currency_id": "ZAR"}}"#;
        let ac: AllowanceCharge = serde_json::from_str(json).unwrap();
        assert!(ac.charge_indicator.is_none());
        assert!(ac.amount.is_some());
    }
}
