/// ORD-R019 (Fatal): Seller party legal entity registration should be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R019".into(),
        description: "Seller party legal entity registration should be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — legal entity registration cannot be verified".into()),
                Some(party) => {
                    if party.party_legal_entity.is_empty() {
                        Err("Seller party has no PartyLegalEntity — legal entity registration should be present".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    }
}
