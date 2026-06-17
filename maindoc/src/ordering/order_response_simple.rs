// UBL OrderResponseSimple document (UBL 2.5)
// DEPRECATED — A simple accept/reject response to an entire Order.
// Superseded by OrderResponse without OrderLines.
// Reference: xsd/maindoc/UBL-OrderResponseSimple-2.5.xsd

use serde::{Deserialize, Serialize};
use ubl_common::cac::*;
use ubl_common::cbc::*;

/// (Deprecated) Simple accept/reject response to an Order.
/// Use OrderResponse without OrderLines instead.
/// UBL element: OrderResponseSimple
#[deprecated(note = "Deprecated in UBL 2.5. Use OrderResponse without OrderLines instead.")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderResponseSimple {
    // === Document Metadata (BBIE) ===
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,

    /// Sender-assigned document identifier (required).
    pub id: ID,
    #[deprecated(note = "Deprecated in UBL 2.5")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,

    /// Date this response was issued (required).
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,

    /// Whether the order was accepted (true) or rejected (false). Required.
    pub accepted_indicator: AcceptedIndicator,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rejection_note: Vec<RejectionNote>,

    // === Accounting (BBIE) ===
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<CustomerReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<AccountingCost>,

    // === Document References (ASBIE: CAC) ===
    /// Reference to the Order being responded to (required, exactly 1).
    pub order_reference: OrderReference,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_change_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<DocumentReference>,

    // === Signature (ASBIE: CAC) ===
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,

    // === Parties (ASBIE: CAC) ===
    /// The seller (required).
    pub seller_supplier_party: SupplierParty,
    /// The buyer (required).
    pub buyer_customer_party: CustomerParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_customer_party: Option<CustomerParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_supplier_party: Option<SupplierParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_customer_party: Option<CustomerParty>,

    // === Beneficiary (ASBIE: CAC) ===
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beneficiary_party: Vec<Party>,
}
