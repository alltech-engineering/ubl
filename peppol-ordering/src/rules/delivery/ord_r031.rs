/// ORD-R031 (Warning): Delivery location address should be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R031".into(),
        description: "Delivery location address should be present if delivery is specified".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, delivery) in inv.delivery.iter().enumerate() {
                    if delivery.delivery_address.is_none() {
                        return Err(format!(
                            "Delivery[{}] has no delivery address — location should be specified",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    }
}
