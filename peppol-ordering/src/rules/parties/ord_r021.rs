/// ORD-R021 (Fatal): Seller party tax scheme must include VAT if applicable
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R021".into(),
        description: "Seller party tax scheme must include VAT if applicable".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — tax scheme cannot be verified".into()),
                Some(party) => {
                    let has_vat = party.party_tax_scheme.iter().any(|pts| {
                        pts.tax_scheme.id.as_ref().map(|id| id.value() == "VAT").unwrap_or(false)
                    });
                    if !has_vat {
                        Err("Seller party has no VAT tax scheme — VAT registration should be present when applicable".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    }
}
