/// ORD-R009 (Fatal): OrderDocumentReference is required if this is a change order
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R009".into(),
        description: "OrderDocumentReference is required if this is a change order".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // Check if order type code indicates a change order (code "220" per UNCL 1001)
                let is_change_order = inv.order_type_code.as_ref()
                    .map(|c| c.value() == "230")
                    .unwrap_or(false);
                if is_change_order && inv.order_document_reference.is_empty() {
                    Err("Order is a change order but no OrderDocumentReference is provided — must reference the original order".into())
                } else {
                    Ok(())
                }
            })
        },
    }
}
