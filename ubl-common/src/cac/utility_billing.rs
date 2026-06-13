// UBL SubscriberConsumption — consumption data for utility billing.
// UBL element: cac:SubscriberConsumption

use crate::cbc::*;
use serde::{Deserialize, Serialize};

/// A class to describe the consumption of a utility by a subscriber.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriberConsumption {
    pub consumption_id: Option<ConsumptionID>,
    #[serde(default)]
    pub note: Vec<Note>,
    pub utility_statement_type_code: Option<StatementTypeCode>,
    pub total_consumed_quantity: Option<TotalConsumedQuantity>,
    pub consumption_level_code: Option<Code>,
    pub consumption_level: Option<Text>,
    #[serde(default)]
    pub description: Vec<Description>,
}

/// A class to describe a main on-account payment for a utility statement.
/// UBL element: cac:MainOnAccountPayment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MainOnAccountPayment {
    #[serde(default)]
    pub on_account_payment: Vec<OnAccountPayment>,
}

/// A class to describe an on-account payment amount.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnAccountPayment {
    pub estimated_consumed_quantity: Option<EstimatedConsumedQuantity>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub payment_terms: Vec<PaymentTerms>,
    #[serde(default)]
    pub payment_means: Vec<PaymentMeans>,
}

use crate::cac::payment::{PaymentMeans, PaymentTerms};
