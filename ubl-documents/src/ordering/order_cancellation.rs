// UBL OrderCancellation document (UBL 2.5)
// A document used to cancel an entire Order.
// Reference: xsd/maindoc/UBL-OrderCancellation-2.5.xsd

use serde::{Deserialize, Serialize};
use ubl_common::cac::*;
use ubl_common::cbc::*;

/// A cancellation of an existing Purchase Order.
/// UBL element: OrderCancellation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderCancellation {
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

    /// Date this cancellation was issued (required).
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    /// The reason for cancellation (required, at least 1).
    pub cancellation_note: Vec<CancellationNote>,

    // === Document References (ASBIE: CAC) ===
    /// Reference to the Order(s) being cancelled (required, at least 1).
    pub order_reference: Vec<OrderReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_document_reference: Option<Box<DocumentReference>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<DocumentReference>,

    // === Contract & Signature (ASBIE: CAC) ===
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract: Vec<Contract>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,

    // === Parties (ASBIE: CAC) ===
    /// The buyer (required).
    pub buyer_customer_party: CustomerParty,
    /// The seller (required).
    pub seller_supplier_party: SupplierParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_customer_party: Option<CustomerParty>,

    // === Beneficiary (ASBIE: CAC) ===
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beneficiary_party: Vec<Party>,
}
