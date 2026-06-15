/// ORD-R030 (Warning): Delivery information should be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R030".into(),
        description: "Delivery information should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.delivery.is_empty() {
                    Err("No delivery information provided — consider specifying delivery details".into())
                } else {
                    Ok(())
                }
            })
        },
    }
}
