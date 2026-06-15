/// ORD-R032 (Warning): Requested delivery period should be specified
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R032".into(),
        description: "Requested delivery period should be specified".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, delivery) in inv.delivery.iter().enumerate() {
                    if delivery.requested_delivery_period.is_none() {
                        return Err(format!(
                            "Delivery[{}] has no requested delivery period — expected delivery date should be specified",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    }
}
