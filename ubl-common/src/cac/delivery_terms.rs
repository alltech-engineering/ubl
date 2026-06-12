// DeliveryTerms — UBL CAC aggregate
//
// UBL 2.5 DeliveryTerms fields:
//   CBC: ID (0..1), SpecialTerms (0..*), LossRiskResponsibilityCode (0..1),
//        LossRisk (0..*), Amount (0..1)
//   CAC: DeliveryLocation (0..1), AllowanceCharge (0..1)
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryTerms {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub special_terms: Vec<SpecialTerms>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss_risk_responsibility_code: Option<LossRiskResponsibilityCode>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub loss_risk: Vec<LossRisk>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
}
