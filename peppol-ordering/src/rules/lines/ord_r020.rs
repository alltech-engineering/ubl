/// ORD-R020 (Fatal): At least one order line must be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R020".into(),
        description: "At least one order line must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.order_line.is_empty() {
                    Err("Order has no line items — at least one order line is required".into())
                } else {
                    Ok(())
                }
            })
        },
    }
}
