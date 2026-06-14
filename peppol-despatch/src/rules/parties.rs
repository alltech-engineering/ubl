// Peppol BIS Despatch Advice 3.0 — Party Business Rules
//
// Validates shipper (DespatchSupplierParty) and receiver (DeliveryCustomerParty)
// party information for Despatch Advice.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::despatch::DespatchAdvice;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<DespatchAdvice>) {
    // ── DESP-R011 (Fatal): DeliveryCustomerParty must be present ─────────
    engine.add_rule(Rule {
        id: "DESP-R011".into(),
        description: "DeliveryCustomerParty must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.delivery_customer_party {
                None => {
                    Err("DeliveryCustomerParty is missing — receiver information is required"
                        .into())
                }
                Some(customer) => match &customer.party {
                    None => Err(
                        "DeliveryCustomerParty has no party — receiver details are required".into(),
                    ),
                    Some(party) => {
                        if party.party_name.is_empty() {
                            Err("Receiver party name is empty — a receiver name is required"
                                .into())
                        } else {
                            Ok(())
                        }
                    }
                },
            })
        },
    });

    // ── DESP-R012 (Fatal): Country must be present in postal addresses ───
    engine.add_rule(Rule {
        id: "DESP-R012".into(),
        description: "Country must be present in postal addresses".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // Check shipper country
                if let Some(supplier) = &inv.despatch_supplier_party {
                    if let Some(party) = &supplier.party {
                        if let Some(addr) = &party.postal_address {
                            if addr.country.is_none() {
                                return Err(
                                    "Shipper postal address does not include a country".into()
                                );
                            }
                        }
                    }
                }
                // Check receiver country
                if let Some(customer) = &inv.delivery_customer_party {
                    if let Some(party) = &customer.party {
                        if let Some(addr) = &party.postal_address {
                            if addr.country.is_none() {
                                return Err(
                                    "Receiver postal address does not include a country".into()
                                );
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R013 (Fatal): DespatchSupplierParty must be present ─────────
    engine.add_rule(Rule {
        id: "DESP-R013".into(),
        description: "DespatchSupplierParty must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.despatch_supplier_party {
                None => Err(
                    "DespatchSupplierParty is missing — shipper information is required".into(),
                ),
                Some(supplier) => match &supplier.party {
                    None => Err(
                        "DespatchSupplierParty has no party — shipper details are required".into(),
                    ),
                    Some(party) => {
                        if party.party_name.is_empty() {
                            Err("Shipper party name is empty — a shipper name is required".into())
                        } else {
                            Ok(())
                        }
                    }
                },
            })
        },
    });

    // ── DESP-R014 (Warning): Shipper party identification should be present ─
    engine.add_rule(Rule {
        id: "DESP-R014".into(),
        description: "Shipper party identification should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(supplier) = &inv.despatch_supplier_party {
                    if let Some(party) = &supplier.party {
                        if party.party_identification.is_empty() {
                            return Err(
                                "Shipper party has no identification — party identification should be provided".into(),
                            );
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R015 (Fatal): Shipper postal address must be present ────────
    engine.add_rule(Rule {
        id: "DESP-R015".into(),
        description: "Shipper postal address must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(supplier) = &inv.despatch_supplier_party {
                    if let Some(party) = &supplier.party {
                        if party.postal_address.is_none() {
                            return Err(
                                "Shipper postal address is missing — a postal address is required"
                                    .into(),
                            );
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R016 (Warning): Receiver party identification should be present ─
    engine.add_rule(Rule {
        id: "DESP-R016".into(),
        description: "Receiver party identification should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(customer) = &inv.delivery_customer_party {
                    if let Some(party) = &customer.party {
                        if party.party_identification.is_empty() {
                            return Err(
                                "Receiver party has no identification — party identification should be provided".into(),
                            );
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R017 (Fatal): Receiver postal address must be present ───────
    engine.add_rule(Rule {
        id: "DESP-R017".into(),
        description: "Receiver postal address must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(customer) = &inv.delivery_customer_party {
                    if let Some(party) = &customer.party {
                        if party.postal_address.is_none() {
                            return Err(
                                "Receiver postal address is missing — a postal address is required"
                                    .into(),
                            );
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R018 (Warning): Shipper city name should be present ────────
    engine.add_rule(Rule {
        id: "DESP-R018".into(),
        description: "Shipper city name should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(supplier) = &inv.despatch_supplier_party {
                    if let Some(party) = &supplier.party {
                        if let Some(addr) = &party.postal_address {
                            if addr.city_name.is_none() {
                                return Err(
                                    "Shipper postal address has no city name".into(),
                                );
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R019 (Warning): Receiver city name should be present ────────
    engine.add_rule(Rule {
        id: "DESP-R019".into(),
        description: "Receiver city name should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(customer) = &inv.delivery_customer_party {
                    if let Some(party) = &customer.party {
                        if let Some(addr) = &party.postal_address {
                            if addr.city_name.is_none() {
                                return Err(
                                    "Receiver postal address has no city name".into(),
                                );
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R020 (Warning): BuyerCustomerParty if present must have name ─
    engine.add_rule(Rule {
        id: "DESP-R020".into(),
        description: "BuyerCustomerParty if present must have a party name".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(buyer) = &inv.buyer_customer_party {
                    if let Some(party) = &buyer.party {
                        if party.party_name.is_empty() {
                            return Err(
                                "BuyerCustomerParty has no party name — a buyer name should be provided".into(),
                            );
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R021 (Warning): SellerSupplierParty if present must have name ─
    engine.add_rule(Rule {
        id: "DESP-R021".into(),
        description: "SellerSupplierParty if present must have a party name".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(seller) = &inv.seller_supplier_party {
                    if let Some(party) = &seller.party {
                        if party.party_name.is_empty() {
                            return Err(
                                "SellerSupplierParty has no party name — a seller name should be provided".into(),
                            );
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R022 (Warning): OriginatorCustomerParty if present must have name ─
    engine.add_rule(Rule {
        id: "DESP-R022".into(),
        description: "OriginatorCustomerParty if present must have a party name".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(originator) = &inv.originator_customer_party {
                    if let Some(party) = &originator.party {
                        if party.party_name.is_empty() {
                            return Err(
                                "OriginatorCustomerParty has no party name — an originator name should be provided".into(),
                            );
                        }
                    }
                }
                Ok(())
            })
        },
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use peppol_common::rules::RuleEngine;
    use std::sync::Arc;
    use ubl_documents::despatch::DespatchAdvice;

    fn minimal_despatch() -> DespatchAdvice {
        serde_json::from_str(
            r#"{
            "id": {"value": "DES-001"},
            "issue_date": "2026-06-13",
            "despatch_advice_type_code": {"value": "delivery"},
            "order_reference": [{"id": {"value": "ORD-001"}}],
            "despatch_supplier_party": {
                "party": {
                    "party_name": [{"name": "Shipper Ltd"}],
                    "party_identification": [{"id": {"value": "9933:shipper"}}],
                    "postal_address": {
                        "street_name": "1 Dock Rd",
                        "city_name": "Durban",
                        "country": {"identification_code": {"value": "ZA"}}
                    }
                }
            },
            "delivery_customer_party": {
                "party": {
                    "party_name": [{"name": "Receiver Ltd"}],
                    "party_identification": [{"id": {"value": "9933:receiver"}}],
                    "postal_address": {
                        "street_name": "2 Warehouse St",
                        "city_name": "Cape Town",
                        "country": {"identification_code": {"value": "ZA"}}
                    }
                }
            },
            "despatch_line": [{
                "id": {"value": "1"},
                "delivered_quantity": {"value": "10", "unit_code": "EA"},
                "item": {"name": "Widget"},
                "order_line_reference": [{"line_id": {"value": "1"}}]
            }]
        }"#,
        )
        .unwrap()
    }

    #[test]
    fn test_missing_shipper_fails() {
        let mut despatch = minimal_despatch();
        despatch.despatch_supplier_party = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(despatch));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "DESP-R013"));
    }

    #[test]
    fn test_missing_receiver_fails() {
        let mut despatch = minimal_despatch();
        despatch.delivery_customer_party = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(despatch));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "DESP-R011"));
    }

    #[test]
    fn test_missing_country_fails() {
        let mut despatch = minimal_despatch();
        // Remove country from both shipper and receiver
        if let Some(ref mut supplier) = despatch.despatch_supplier_party {
            if let Some(ref mut party) = supplier.party {
                if let Some(ref mut addr) = party.postal_address {
                    addr.country = None;
                }
            }
        }
        if let Some(ref mut customer) = despatch.delivery_customer_party {
            if let Some(ref mut party) = customer.party {
                if let Some(ref mut addr) = party.postal_address {
                    addr.country = None;
                }
            }
        }
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(despatch));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "DESP-R012"));
    }
}
