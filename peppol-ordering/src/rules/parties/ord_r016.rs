/// ORD-R016 (Warning): Buyer contact should be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R016".into(),
        description: "Buyer contact should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.buyer_customer_party.buyer_contact.is_none() {
                    Err("Buyer contact is not present — should be provided".into())
                } else {
                    Ok(())
                }
            })
        },
    }
}
