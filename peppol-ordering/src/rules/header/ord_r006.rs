/// ORD-R006 (Fatal): A buyer reference should be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R006".into(),
        description: "A buyer reference (quotation or prior order) should be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                let has_quotation = inv.quotation_document_reference.is_some();
                let has_prior_order = !inv.order_document_reference.is_empty();
                if has_quotation || has_prior_order {
                    Ok(())
                } else {
                    Err("No buyer reference provided — a quotation or prior order reference should be present".into())
                }
            })
        },
    }
}
