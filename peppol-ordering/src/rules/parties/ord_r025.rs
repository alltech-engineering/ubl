/// ORD-R025 (Fatal): Seller postal address street and city must be present
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R025".into(),
        description: "Seller postal address street and city must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — postal address cannot be verified".into()),
                Some(party) => match &party.postal_address {
                    None => Err("Seller postal address is missing — street and city are required".into()),
                    Some(addr) => {
                        if addr.street_name.is_none() {
                            Err("Seller postal address has no street name — street is required".into())
                        } else if addr.city_name.is_none() {
                            Err("Seller postal address has no city — city is required".into())
                        } else {
                            Ok(())
                        }
                    }
                },
            })
        },
    }
}
