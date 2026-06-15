/// ORESP-R003 (Fatal): Must reference original Order
/// Checks both order_reference (cac:OrderReference) and
/// order_document_reference (cac:OrderDocumentReference) for an ID.
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::OrderResponse;

pub fn rule(inv: &Arc<OrderResponse>) -> Rule {
    Rule {
        id: "ORESP-R003".into(),
        description: "Must reference the original Order via OrderReference or OrderDocumentReference with ID".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                let has_order_ref = inv.order_reference.iter().any(|r| {
                    r.id.as_ref().map(|id| !id.value().is_empty()).unwrap_or(false)
                });
                let has_doc_ref = inv.order_document_reference.iter().any(|dr| {
                    dr.id.as_ref().map(|id| !id.value().is_empty()).unwrap_or(false)
                });
                if has_order_ref || has_doc_ref {
                    Ok(())
                } else {
                    Err("No reference to the original Order — an OrderReference or OrderDocumentReference with an ID is required".into())
                }
            })
        },
    }
}
