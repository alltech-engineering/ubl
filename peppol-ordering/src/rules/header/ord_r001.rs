/// ORD-R001 (Fatal): Order ID must be present and non-empty
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R001".into(),
        description: "Order ID must be present and non-empty".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.id.value().is_empty() {
                    Err("Order ID is empty — a non-empty order identifier is required".into())
                } else {
                    Ok(())
                }
            })
        },
    }
}
