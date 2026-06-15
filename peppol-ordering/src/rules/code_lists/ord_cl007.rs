/// ORD-CL007 (Fatal): DocumentTypeCode on AdditionalDocumentReference must be valid
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-CL007".into(),
        description: "DocumentTypeCode on AdditionalDocumentReference must be valid (non-empty)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, doc_ref) in inv.additional_document_reference.iter().enumerate() {
                    match &doc_ref.document_type_code {
                        None => {
                            return Err(format!(
                                "AdditionalDocumentReference[{}] has no DocumentTypeCode — a valid type code is required",
                                i + 1
                            ));
                        }
                        Some(code) if code.value().is_empty() => {
                            return Err(format!(
                                "AdditionalDocumentReference[{}] DocumentTypeCode is empty — must be a valid UNCL 1001 code",
                                i + 1
                            ));
                        }
                        Some(_) => {}
                    }
                }
                Ok(())
            })
        },
    }
}
