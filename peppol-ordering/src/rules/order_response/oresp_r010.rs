/// ORESP-R010 (Fatal): Seller party identification must be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::OrderResponse;

pub fn rule(inv: &Arc<OrderResponse>) -> Rule {
    Rule {
        id: "ORESP-R010".into(),
        description: "Seller party identification must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — party identification cannot be verified".into()),
                Some(party) => {
                    if party.party_identification.is_empty() {
                        Err("Seller has no PartyIdentification — at least one identifier is required".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    }
}
