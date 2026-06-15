/// ORD-R018 (Fatal): Buyer party legal entity registration should be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R018".into(),
        description: "Buyer party legal entity registration should be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — legal entity registration cannot be verified".into()),
                Some(party) => {
                    if party.party_legal_entity.is_empty() {
                        Err("Buyer party has no PartyLegalEntity — legal entity registration should be present".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    }
}
