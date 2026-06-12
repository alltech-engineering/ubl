// PaymentMeans — UBL CAC aggregate (Tier 1 stub)
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentMeans {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_means_code: Option<PaymentMeansCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_due_date: Option<DueDate>,
}
