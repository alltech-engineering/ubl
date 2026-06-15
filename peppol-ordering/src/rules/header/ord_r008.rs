/// ORD-R008 (Warning): QuotationDocumentReference should be present for traceability
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R008".into(),
        description: "QuotationDocumentReference should be present for traceability".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.quotation_document_reference.is_none() {
                    Err(
                        "No quotation document reference — consider providing for traceability"
                            .into(),
                    )
                } else {
                    Ok(())
                }
            })
        },
    }
}
