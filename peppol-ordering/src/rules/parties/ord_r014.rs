/// ORD-R014 (Warning): Buyer postal address should include country
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R014".into(),
        description: "Buyer postal address should include country".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — postal address cannot be verified".into()),
                Some(party) => match &party.postal_address {
                    None => Err("Buyer postal address is missing".into()),
                    Some(addr) => {
                        if addr.country.is_none() {
                            Err("Buyer postal address does not include a country".into())
                        } else {
                            Ok(())
                        }
                    }
                },
            })
        },
    }
}
