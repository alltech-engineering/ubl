/// ORD-R012 (Fatal): Seller party must be present and have name
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R012".into(),
        description: "Seller party must be present and have name".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — seller name cannot be verified".into()),
                Some(party) => {
                    if party.party_name.is_empty() {
                        Err("Seller party name is empty — a seller name is required".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    }
}
