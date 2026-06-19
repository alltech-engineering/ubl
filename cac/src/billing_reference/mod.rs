use serde::{Deserialize, Serialize};

include!("line.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to define a reference to a billing document.
///
/// UBL Dictionary Entry Name: `Billing Reference. Details`
///
/// Generated from XSD type `BillingReferenceType`.
pub struct BillingReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A reference to an invoice.
    #[serde(default, rename = "InvoiceDocumentReference")]
    pub invoice_document_reference: Option<crate::DocumentReference>,
/// A reference to a self billed invoice.
    #[serde(default, rename = "SelfBilledInvoiceDocumentReference")]
    pub self_billed_invoice_document_reference: Option<crate::DocumentReference>,
/// A reference to a credit note.
    #[serde(default, rename = "CreditNoteDocumentReference")]
    pub credit_note_document_reference: Option<crate::DocumentReference>,
/// A reference to a self billed credit note.
    #[serde(default, rename = "SelfBilledCreditNoteDocumentReference")]
    pub self_billed_credit_note_document_reference: Option<crate::DocumentReference>,
/// A reference to a debit note.
    #[serde(default, rename = "DebitNoteDocumentReference")]
    pub debit_note_document_reference: Option<crate::DocumentReference>,
/// A reference to a billing reminder.
    #[serde(default, rename = "ReminderDocumentReference")]
    pub reminder_document_reference: Option<crate::DocumentReference>,
/// A reference to an additional document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Option<crate::DocumentReference>,
/// A reference to a transaction line in the billing document.
    #[serde(default, rename = "BillingReferenceLine")]
    pub billing_reference_line: Vec<BillingReferenceLine>,
}
