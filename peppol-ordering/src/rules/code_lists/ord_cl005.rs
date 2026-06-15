/// ORD-CL005 (Fatal): Country codes must be valid ISO 3166-1 alpha-2
use peppol_common::rules::{Rule, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn rule(inv: &Arc<Order>) -> Rule {
    Rule {
        id: "ORD-CL005".into(),
        description: "Country codes must be valid ISO 3166-1 alpha-2".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // Validate destination country if present
                if let Some(dest_country) = &inv.destination_country {
                    if let Some(code) = &dest_country.identification_code {
                        let v = code.value();
                        if v.len() != 2 || !v.chars().all(|c| c.is_ascii_uppercase()) {
                            return Err(format!(
                                "Destination country code '{}' is not a valid ISO 3166-1 alpha-2 code",
                                v
                            ));
                        }
                    }
                }

                // Validate buyer country if present
                if let Some(party) = &inv.buyer_customer_party.party {
                    if let Some(addr) = &party.postal_address {
                        if let Some(country) = &addr.country {
                            if let Some(code) = &country.identification_code {
                                let v = code.value();
                                if v.len() != 2 || !v.chars().all(|c| c.is_ascii_uppercase()) {
                                    return Err(format!(
                                        "Buyer country code '{}' is not a valid ISO 3166-1 alpha-2 code",
                                        v
                                    ));
                                }
                            }
                        }
                    }
                }

                // Validate seller country if present
                if let Some(party) = &inv.seller_supplier_party.party {
                    if let Some(addr) = &party.postal_address {
                        if let Some(country) = &addr.country {
                            if let Some(code) = &country.identification_code {
                                let v = code.value();
                                if v.len() != 2 || !v.chars().all(|c| c.is_ascii_uppercase()) {
                                    return Err(format!(
                                        "Seller country code '{}' is not a valid ISO 3166-1 alpha-2 code",
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
