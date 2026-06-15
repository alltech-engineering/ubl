/// ORD-R035 (Warning): Delivery party should be specified for ship-to scenarios
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R035".into(),
        description: "Delivery party should be specified for ship-to scenarios".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, delivery) in inv.delivery.iter().enumerate() {
                    if delivery.delivery_party.is_none() {
                        return Err(format!(
                            "Delivery[{}] has no delivery party — should be specified for ship-to scenarios",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    }
}
