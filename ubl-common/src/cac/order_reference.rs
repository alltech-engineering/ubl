// OrderReference — UBL CAC aggregate
// References an order document.
use crate::cbc::*;

/// A reference to an order.
/// UBL element: cac:OrderReference
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_order_id: Option<SalesOrderID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<IssueDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<CustomerReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type_code: Option<OrderTypeCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<DocumentReference>,
}

use super::document_reference::DocumentReference;
