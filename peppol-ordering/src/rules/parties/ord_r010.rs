/// ORD-R010 (Fatal): Buyer party must be present and have name
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R010".into(),
        description: "Buyer party must be present and have name".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — buyer name cannot be verified".into()),
                Some(party) => {
                    if party.party_name.is_empty() {
                        Err("Buyer party name is empty — a buyer name is required".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    }
}
