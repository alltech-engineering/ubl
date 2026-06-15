/// ORD-R023 (Warning): Seller electronic address (endpoint) should be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R023".into(),
        description: "Seller electronic address (endpoint) should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — endpoint cannot be verified".into()),
                Some(party) => {
                    if party.endpoint_id.is_none() {
                        Err("Seller party has no EndpointID — electronic address should be present".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    }
}
