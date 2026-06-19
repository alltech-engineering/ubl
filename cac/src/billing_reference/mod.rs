use serde::{Deserialize, Serialize};

include!("line.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct BillingReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "InvoiceDocumentReference")]
    pub invoice_document_reference: Option<crate::DocumentReference>,
    #[serde(default, rename = "SelfBilledInvoiceDocumentReference")]
    pub self_billed_invoice_document_reference: Option<crate::DocumentReference>,
    #[serde(default, rename = "CreditNoteDocumentReference")]
    pub credit_note_document_reference: Option<crate::DocumentReference>,
    #[serde(default, rename = "SelfBilledCreditNoteDocumentReference")]
    pub self_billed_credit_note_document_reference: Option<crate::DocumentReference>,
    #[serde(default, rename = "DebitNoteDocumentReference")]
    pub debit_note_document_reference: Option<crate::DocumentReference>,
    #[serde(default, rename = "ReminderDocumentReference")]
    pub reminder_document_reference: Option<crate::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Option<crate::DocumentReference>,
    #[serde(default, rename = "BillingReferenceLine")]
    pub billing_reference_line: Vec<BillingReferenceLine>,
}
