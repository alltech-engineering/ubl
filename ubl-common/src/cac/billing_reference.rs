// BillingReference — UBL CAC aggregate
// References a billing document (invoice, credit note, etc.)
use crate::cbc::*;

/// A reference to a billing document.
/// UBL element: cac:BillingReference
#[derive(Debug, Clone, Partialserde::Serialize, serde::Deserialize)]
pub struct BillingReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_document_reference: Option<DocumentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_billed_invoice_document_reference: Option<DocumentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note_document_reference: Option<DocumentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_billed_credit_note_document_reference: Option<DocumentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_note_document_reference: Option<DocumentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder_document_reference: Option<DocumentReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document_reference: Option<DocumentReference>,
}

use super::document_reference::DocumentReference;
