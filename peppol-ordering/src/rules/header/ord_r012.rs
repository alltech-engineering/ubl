/// ORD-R012 (Warning): ValidityDuration should be specified for time-limited orders
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R012".into(),
        description: "Validity period should be specified for time-limited orders".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.validity_period.is_empty() {
                    Err("Order has no validity period — consider specifying validity duration for time-limited orders".into())
                } else {
                    Ok(())
                }
            })
        },
    }
}
