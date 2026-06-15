/// ORD-R039 (Fatal): Delivery address country must be ISO 3166-1 alpha-2
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R039".into(),
        description: "Delivery address country must be valid ISO 3166-1 alpha-2".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, delivery) in inv.delivery.iter().enumerate() {
                    if let Some(addr) = &delivery.delivery_address {
                        if let Some(country) = &addr.country {
                            if let Some(code) = &country.identification_code {
                                let v = code.value();
                                if v.len() != 2 || !v.chars().all(|c| c.is_ascii_uppercase()) {
                                    return Err(format!(
                                        "Delivery[{}] address country code '{}' is not a valid ISO 3166-1 alpha-2 code",
                                        i + 1,
                                        v
                                    ));
                                }
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    }
}
