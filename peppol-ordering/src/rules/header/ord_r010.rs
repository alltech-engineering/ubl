/// ORD-R010 (Warning): AdditionalDocumentReference should have document type code
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R010".into(),
        description: "AdditionalDocumentReference should have document type code".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, doc_ref) in inv.additional_document_reference.iter().enumerate() {
                    if doc_ref.document_type_code.is_none() {
                        return Err(format!(
                            "AdditionalDocumentReference[{}] has no DocumentTypeCode — type should be specified",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    }
}
