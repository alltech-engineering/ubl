/// ORD-R022 (Warning): Buyer electronic address (endpoint) should be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R022".into(),
        description: "Buyer electronic address (endpoint) should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — endpoint cannot be verified".into()),
                Some(party) => {
                    if party.endpoint_id.is_none() {
                        Err("Buyer party has no EndpointID — electronic address should be present".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    }
}
