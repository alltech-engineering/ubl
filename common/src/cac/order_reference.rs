// UBL Order and Billing Reference aggregates.

use crate::cac::document::DocumentReference;
use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderReference {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub sales_order_id: Option<SalesOrderID>,
    #[serde(default)]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default)]
    pub uuid: Option<UUID>,
    #[serde(default)]
    pub issue_date: Option<IssueDate>,
    #[serde(default)]
    pub issue_time: Option<IssueTime>,
    #[serde(default)]
    pub customer_reference: Option<CustomerReference>,
    #[serde(default)]
    pub order_type_code: Option<OrderTypeCode>,
    #[serde(default)]
    pub document_reference: Option<Box<DocumentReference>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BillingReference {
    #[serde(default)]
    pub invoice_document_reference: Option<Box<DocumentReference>>,
    #[serde(default)]
    pub self_billed_invoice_document_reference: Option<Box<DocumentReference>>,
    #[serde(default)]
    pub credit_note_document_reference: Option<Box<DocumentReference>>,
    #[serde(default)]
    pub self_billed_credit_note_document_reference: Option<Box<DocumentReference>>,
    #[serde(default)]
    pub debit_note_document_reference: Option<Box<DocumentReference>>,
    #[serde(default)]
    pub reminder_document_reference: Option<Box<DocumentReference>>,
    #[serde(default)]
    pub additional_document_reference: Option<Box<DocumentReference>>,
    #[serde(default)]
    pub billing_reference_line: Vec<BillingReferenceLine>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BillingReferenceLine {
    pub id: ID,
    #[serde(default)]
    pub amount: Option<Amount>,
    #[serde(default)]
    pub allowance_charge: Vec<AllowanceCharge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectReference {
    pub id: ID,
    #[serde(default)]
    pub uuid: Option<UUID>,
    #[serde(default)]
    pub issue_date: Option<IssueDate>,
}

use crate::cac::allowance::AllowanceCharge;
