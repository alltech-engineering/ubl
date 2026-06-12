// UBL 2.5 Despatch & Fulfilment Document Types
//
// Reference: https://docs.oasis-open.org/ubl/cs01-UBL-2.5/UBL-2.5.html
//
// Six document types covering the physical delivery and receipt of goods:
//   - DespatchAdvice:     Notification of goods shipped
//   - ReceiptAdvice:      Confirmation of goods received
//   - FulfilmentCancellation: Cancel a despatch/receipt advice
//   - DeliveryNote:       Accompanying delivery document
//   - PackingList:        Distribution of goods in packages
//   - InstructionForReturns: Instructions for returning goods

use serde::{Deserialize, Serialize};
use ubl_common::cbc::*;
use ubl_common::cac::*;

// ══════════════════════════════════════════════════════════════════════
// DespatchAdvice
// ══════════════════════════════════════════════════════════════════════

/// Notification of goods shipped. Describes the despatch or delivery
/// of goods and services. Counterpart to ReceiptAdvice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DespatchAdvice {
    // ── Document metadata ──
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_status_code: Option<DocumentStatusCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch_advice_type_code: Option<DespatchAdviceTypeCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<AccountingCost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_count_numeric: Option<LineCountNumeric>,

    // ── Document references ──
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_reference: Vec<OrderReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,

    // ── Parties ──
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch_supplier_party: Option<SupplierParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<SupplierParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_party: Option<Party>,

    // ── Body ──
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment: Option<Shipment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub despatch_line: Vec<DespatchLine>,
}

// ══════════════════════════════════════════════════════════════════════
// ReceiptAdvice
// ══════════════════════════════════════════════════════════════════════

/// Confirmation of goods received. Describes the receipt of goods and
/// services. May be a reply to a DespatchAdvice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiptAdvice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_status_code: Option<DocumentStatusCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_advice_type_code: Option<ReceiptAdviceTypeCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_acceptance_code: Option<DeliveryAcceptanceCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason_code: Option<RejectReasonCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reject_reason: Vec<RejectReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_action_code: Option<RejectActionCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_count_numeric: Option<LineCountNumeric>,

    // ── Document references ──
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_reference: Vec<OrderReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub despatch_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,

    // ── Parties ──
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch_supplier_party: Option<SupplierParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<SupplierParty>,

    // ── Body ──
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment: Option<Shipment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub receipt_line: Vec<ReceiptLine>,
}

// ══════════════════════════════════════════════════════════════════════
// FulfilmentCancellation
// ══════════════════════════════════════════════════════════════════════

/// Cancels a previously issued DespatchAdvice or ReceiptAdvice.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FulfilmentCancellation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cancellation_note: Vec<CancellationNote>,

    // ── Document references ──
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub despatch_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub receipt_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_reference: Vec<OrderReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract: Vec<Contract>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,

    // ── Parties ──
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<SupplierParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch_supplier_party: Option<SupplierParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_party: Option<Party>,
}

// ══════════════════════════════════════════════════════════════════════
// DeliveryNote
// ══════════════════════════════════════════════════════════════════════

/// A document accompanying a delivery of goods. Similar to
/// DespatchAdvice but typically travels with the physical shipment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliveryNote {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_status_code: Option<DocumentStatusCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<AccountingCost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_count_numeric: Option<LineCountNumeric>,

    // ── Document references ──
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order_reference: Vec<OrderReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,

    // ── Parties ──
    #[serde(skip_serializing_if = "Option::is_none")]
    pub despatch_supplier_party: Option<SupplierParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<SupplierParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_party: Option<Party>,

    // ── Body ──
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment: Option<Shipment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub despatch_line: Vec<DespatchLine>,
}

// ══════════════════════════════════════════════════════════════════════
// PackingList
// ══════════════════════════════════════════════════════════════════════

/// Describes how goods are packed for a shipment. Lists the
/// distribution of goods across packages/containers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackingList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<IssueDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<Description>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<VersionID>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_instruction: Vec<Text>,

    // ── Parties ──
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consignor_party: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier_party: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_forwarder_party: Option<Party>,

    // ── Body ──
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment: Option<Shipment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_distribution: Vec<DocumentDistribution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
}

// ══════════════════════════════════════════════════════════════════════
// InstructionForReturns
// ══════════════════════════════════════════════════════════════════════

/// Instructions for returning goods. Typically issued by a manufacturer
/// or supplier to a retailer, requesting return of products.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstructionForReturns {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,

    // ── Document references ──
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,

    // ── Parties ──
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_supplier_party: Option<SupplierParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retailer_customer_party: Option<CustomerParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_party: Option<Party>,

    // ── Body ──
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment: Option<Shipment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instruction_for_returns_line: Vec<InstructionForReturnsLine>,
}
