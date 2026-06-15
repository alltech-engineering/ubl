/// ORD-R017 (Warning): Seller contact should be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R017".into(),
        description: "Seller contact should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.seller_supplier_party.seller_contact.is_none() {
                    Err("Seller contact is not present — should be provided".into())
                } else {
                    Ok(())
                }
            })
        },
    }
}
