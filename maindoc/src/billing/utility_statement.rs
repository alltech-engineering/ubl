// UBL UtilityStatement — billing document type.
// UBL element: maindoc:UtilityStatement

use serde::{Deserialize, Serialize};
use ubl_common::cbc::*;
use ubl_common::cac::customer::CustomerParty;
use ubl_common::cac::document::DocumentReference;
use ubl_common::cac::document::Signature;
use ubl_common::cac::party::Party;
use ubl_common::cac::utility_billing::{MainOnAccountPayment, SubscriberConsumption};

/// A periodic utility billing statement (electricity, water, gas, telecom).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UtilityStatement {
    pub id: ID,
    pub copy_indicator: Option<CopyIndicator>,
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    pub issue_time: Option<IssueTime>,
    pub utility_statement_type_code: Option<StatementTypeCode>,
    pub note: Vec<Note>,
    pub document_currency_code: DocumentCurrencyCode,
    pub accounting_cost_code: Option<AccountingCostCode>,
    pub accounting_cost: Option<AccountingCost>,
    pub parent_document_reference: Option<DocumentReference>,
    pub additional_document_reference: Vec<DocumentReference>,
    pub signature: Vec<Signature>,
    pub sender_party: Option<Party>,
    pub receiver_party: Option<Party>,
    pub customer_party: Option<CustomerParty>,
    pub subscriber_party: Option<Party>,
    pub main_on_account_payment: Vec<MainOnAccountPayment>,
    pub subscriber_consumption: Vec<SubscriberConsumption>,
}
