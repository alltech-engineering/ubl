/// ORD-R034 (Fatal): Delivery location country code must be present if delivery is specified
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-R034".into(),
        description: "Delivery location country code must be present if delivery is specified".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, delivery) in inv.delivery.iter().enumerate() {
                    if let Some(addr) = &delivery.delivery_address {
                        match &addr.country {
                            None => {
                                return Err(format!(
                                    "Delivery[{}] address has no country — country code is required",
                                    i + 1
                                ));
                            }
                            Some(country) => {
                                if country.identification_code.is_none() {
                                    return Err(format!(
                                        "Delivery[{}] address country has no identification code — ISO 3166-1 alpha-2 code is required",
                                        i + 1
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
