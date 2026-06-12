// UBL Order and Billing Reference aggregates.

use serde::{Deserialize, Serialize};
use crate::cbc::*;
use crate::cac::document::DocumentReference;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderReference {
    pub id: Option<ID>,
    pub sales_order_id: Option<SalesOrderID>,
    pub copy_indicator: Option<CopyIndicator>,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub order_type_code: Option<OrderTypeCode>,
    pub document_reference: Option<Box<DocumentReference>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BillingReference {
    pub invoice_document_reference: Option<Box<DocumentReference>>,
    pub self_billed_invoice_document_reference: Option<Box<DocumentReference>>,
    pub credit_note_document_reference: Option<Box<DocumentReference>>,
    pub self_billed_credit_note_document_reference: Option<Box<DocumentReference>>,
    pub debit_note_document_reference: Option<Box<DocumentReference>>,
    pub reminder_document_reference: Option<Box<DocumentReference>>,
    pub additional_document_reference: Option<Box<DocumentReference>>,
    pub billing_reference_line: Vec<BillingReferenceLine>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BillingReferenceLine {
    pub id: ID,
    pub amount: Option<Amount>,
    pub allowance_charge: Vec<AllowanceCharge>,
}

use crate::cac::allowance::AllowanceCharge;
