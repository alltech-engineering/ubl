// UBL PrepaidPayment aggregate — payment made in advance.
// UBL element: cac:PrepaidPayment

use crate::cbc::*;
use serde::{Deserialize, Serialize};

/// A class to describe a prepaid payment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrepaidPayment {
    pub id: Option<ID>,
    pub paid_amount: Option<PaidAmount>,
    pub received_date: Option<ReceivedDate>,
    pub paid_date: Option<PaidDate>,
    pub paid_time: Option<PaidTime>,
    pub instruction_id: Option<InstructionID>,
}
