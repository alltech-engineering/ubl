/// ORD-R011 (Fatal): Buyer must have party identification
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R011".into(),
        description: "Buyer must have party identification".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — party identification cannot be verified".into()),
                Some(party) => {
                    if party.party_identification.is_empty() {
                        Err("Buyer has no PartyIdentification — at least one identifier is required".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    }
}
