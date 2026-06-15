/// ORD-R020 (Fatal): Buyer party tax scheme must include VAT if applicable
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R020".into(),
        description: "Buyer party tax scheme must include VAT if applicable".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — tax scheme cannot be verified".into()),
                Some(party) => {
                    let has_vat = party.party_tax_scheme.iter().any(|pts| {
                        pts.tax_scheme.id.as_ref().map(|id| id.value() == "VAT").unwrap_or(false)
                    });
                    if !has_vat {
                        Err("Buyer party has no VAT tax scheme — VAT registration should be present when applicable".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    }
}
