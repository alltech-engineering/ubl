// PaymentMeans — UBL CAC aggregate
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentMeans {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_means_code: Option<PaymentMeansCode>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub payment_means_description: Vec<PaymentMeansDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_due_date: Option<DueDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_channel_code: Option<PaymentChannelCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_rail_id: Option<PaymentRailID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_platform_id: Option<PaymentPlatformID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction_id: Option<InstructionID>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub instruction_note: Vec<InstructionNote>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub payment_id: Vec<PaymentID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_bearer_code: Option<ChargeBearerCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_level_code: Option<ServiceLevelCode>,
}
