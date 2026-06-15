/// ORD-R004 (Warning): Order should have a validity period
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R004".into(),
        description: "Order should have a validity period".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.validity_period.is_empty() {
                    Err("Order has no validity period — consider specifying how long this order is valid".into())
                } else {
                    Ok(())
                }
            })
        },
    }
}
