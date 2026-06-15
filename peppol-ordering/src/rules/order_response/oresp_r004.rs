/// ORESP-R004 (Fatal): Seller party must be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::OrderResponse;

pub fn rule(inv: &Arc<OrderResponse>) -> Rule {
    Rule {
        id: "ORESP-R004".into(),
        description: "Seller party must be present (the responder)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — the responding party is required".into()),
                Some(_) => Ok(()),
            })
        },
    }
}
