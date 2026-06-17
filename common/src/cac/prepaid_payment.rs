// UBL PrepaidPayment aggregate — payment made in advance.
// UBL element: cac:PrepaidPayment

use crate::cbc::*;
use serde::{Deserialize, Serialize};

/// A class to describe a prepaid payment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrepaidPayment {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub paid_amount: Option<PaidAmount>,
    #[serde(default)]
    pub received_date: Option<ReceivedDate>,
    #[serde(default)]
    pub paid_date: Option<PaidDate>,
    #[serde(default)]
    pub paid_time: Option<PaidTime>,
    #[serde(default)]
    pub instruction_id: Option<InstructionID>,
}
