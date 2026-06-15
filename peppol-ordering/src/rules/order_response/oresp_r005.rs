/// ORESP-R005 (Fatal): Buyer party must be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::OrderResponse;

pub fn rule(inv: &Arc<OrderResponse>) -> Rule {
    Rule {
        id: "ORESP-R005".into(),
        description: "Buyer party must be present (the original order sender)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => {
                    Err("Buyer party is missing — the original order sender is required".into())
                }
                Some(_) => Ok(()),
            })
        },
    }
}
