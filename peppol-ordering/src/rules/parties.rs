// Peppol BIS Ordering 3.0 — Party Business Rules
//
// Validates buyer (BuyerCustomerParty) and seller (SellerSupplierParty)
// party information for Purchase Orders.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Order>) {
    // ═══════════════════════════════════════════════════════════════
    // BUYER RULES  (BuyerCustomerParty)
    // ═══════════════════════════════════════════════════════════════

    // ── ORD-R010 (Fatal): Buyer party must be present and have name ───────
    engine.add_rule(Rule {
        id: "ORD-R010".into(),
        description: "Buyer party must be present and have name".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — buyer name cannot be verified".into()),
                Some(party) => {
                    if party.party_name.is_empty() {
                        Err("Buyer party name is empty — a buyer name is required".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ── ORD-R011 (Fatal): Buyer must have party identification ────────────
    engine.add_rule(Rule {
        id: "ORD-R011".into(),
        description: "Buyer must have party identification".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — party identification cannot be verified".into()),
                Some(party) => {
                    if party.party_identification.is_empty() {
                        Err("Buyer has no PartyIdentification — at least one identifier is required".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════
    // SELLER RULES  (SellerSupplierParty)
    // ═══════════════════════════════════════════════════════════════

    // ── ORD-R012 (Fatal): Seller party must be present and have name ──────
    engine.add_rule(Rule {
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
    });

    // ── ORD-R013 (Fatal): Seller must have party identification ───────────
    engine.add_rule(Rule {
        id: "ORD-R013".into(),
        description: "Seller must have party identification".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — party identification cannot be verified".into()),
                Some(party) => {
                    if party.party_identification.is_empty() {
                        Err("Seller has no PartyIdentification — at least one identifier is required".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ── ORD-R014 (Warning): Buyer postal address should include country ────
    engine.add_rule(Rule {
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
    });

    // ── ORD-R015 (Warning): Seller postal address should include country ───
    engine.add_rule(Rule {
        id: "ORD-R015".into(),
        description: "Seller postal address should include country".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — postal address cannot be verified".into()),
                Some(party) => match &party.postal_address {
                    None => Err("Seller postal address is missing".into()),
                    Some(addr) => {
                        if addr.country.is_none() {
                            Err("Seller postal address does not include a country".into())
                        } else {
                            Ok(())
                        }
                    }
                },
            })
        },
    });

    // ── ORD-R016 (Warning): Buyer contact should be present ───────────────
    engine.add_rule(Rule {
        id: "ORD-R016".into(),
        description: "Buyer contact should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.buyer_customer_party.buyer_contact.is_none() {
                    Err("Buyer contact is not present — should be provided".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORD-R017 (Warning): Seller contact should be present ──────────────
    engine.add_rule(Rule {
        id: "ORD-R017".into(),
        description: "Seller contact should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.seller_supplier_party.seller_contact.is_none() {
                    Err("Seller contact is not present — should be provided".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════
    // NEW RULES (ORD-R018 through ORD-R026)
    // ═══════════════════════════════════════════════════════════════

    // ── ORD-R018 (Fatal): Buyer party legal entity registration should be present
    engine.add_rule(Rule {
        id: "ORD-R018".into(),
        description: "Buyer party legal entity registration should be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — legal entity registration cannot be verified".into()),
                Some(party) => {
                    if party.party_legal_entity.is_empty() {
                        Err("Buyer party has no PartyLegalEntity — legal entity registration should be present".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ── ORD-R019 (Fatal): Seller party legal entity registration should be present
    engine.add_rule(Rule {
        id: "ORD-R019".into(),
        description: "Seller party legal entity registration should be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — legal entity registration cannot be verified".into()),
                Some(party) => {
                    if party.party_legal_entity.is_empty() {
                        Err("Seller party has no PartyLegalEntity — legal entity registration should be present".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ── ORD-R020 (Fatal): Buyer party tax scheme must include VAT if applicable
    engine.add_rule(Rule {
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
    });

    // ── ORD-R021 (Fatal): Seller party tax scheme must include VAT if applicable
    engine.add_rule(Rule {
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
    });

    // ── ORD-R022 (Warning): Buyer electronic address (endpoint) should be present
    engine.add_rule(Rule {
        id: "ORD-R022".into(),
        description: "Buyer electronic address (endpoint) should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — endpoint cannot be verified".into()),
                Some(party) => {
                    if party.endpoint_id.is_none() {
                        Err("Buyer party has no EndpointID — electronic address should be present".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ── ORD-R023 (Warning): Seller electronic address (endpoint) should be present
    engine.add_rule(Rule {
        id: "ORD-R023".into(),
        description: "Seller electronic address (endpoint) should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — endpoint cannot be verified".into()),
                Some(party) => {
                    if party.endpoint_id.is_none() {
                        Err("Seller party has no EndpointID — electronic address should be present".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ── ORD-R024 (Fatal): Buyer postal address street and city must be present
    engine.add_rule(Rule {
        id: "ORD-R024".into(),
        description: "Buyer postal address street and city must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — postal address cannot be verified".into()),
                Some(party) => match &party.postal_address {
                    None => Err("Buyer postal address is missing — street and city are required".into()),
                    Some(addr) => {
                        if addr.street_name.is_none() {
                            Err("Buyer postal address has no street name — street is required".into())
                        } else if addr.city_name.is_none() {
                            Err("Buyer postal address has no city — city is required".into())
                        } else {
                            Ok(())
                        }
                    }
                },
            })
        },
    });

    // ── ORD-R025 (Fatal): Seller postal address street and city must be present
    engine.add_rule(Rule {
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
    });

    // ── ORD-R026 (Warning): OriginatorCustomerParty should be present for drop-ship scenarios
    engine.add_rule(Rule {
        id: "ORD-R026".into(),
        description: "OriginatorCustomerParty should be present for drop-ship scenarios".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.originator_customer_party.is_none() {
                    Err("OriginatorCustomerParty is not present — should be specified for drop-ship scenarios".into())
                } else {
                    Ok(())
                }
            })
        },
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use peppol_common::rules::RuleEngine;
    use std::sync::Arc;
    use ubl_common::cbc;
    use ubl_documents::ordering::Order;

    fn minimal_order() -> Order {
        serde_json::from_str(
            r#"{
            "id": {"value": "ORD-001"},
            "issue_date": "2026-06-13",
            "document_currency_code": {"value": "ZAR"},
            "buyer_customer_party": {
                "party": {
                    "party_name": [{"name": "Buyer Ltd"}],
                    "party_identification": [{"id": {"value": "9933:buyer123"}}],
                    "postal_address": {
                        "street_name": "100 Buyer St",
                        "city_name": "Cape Town",
                        "country": {"identification_code": {"value": "ZA"}}
                    }
                }
            },
            "seller_supplier_party": {
                "party": {
                    "party_name": [{"name": "Supplier Corp"}],
                    "party_identification": [{"id": {"value": "9933:supplier456"}}],
                    "postal_address": {
                        "street_name": "200 Supplier Ave",
                        "city_name": "Johannesburg",
                        "country": {"identification_code": {"value": "ZA"}}
                    }
                }
            },
            "order_line": [{
                "id": {"value": "1"},
                "quantity": {"value": "10", "unit_code": "EA"},
                "line_extension_amount": {"value": "1000.00", "currency_id": "ZAR"},
                "item": {"name": "Widget"},
                "price": {"price_amount": {"value": "100.00", "currency_id": "ZAR"}}
            }]
        }"#,
        )
        .unwrap()
    }

    #[test]
    fn test_missing_buyer_party_fails() {
        let mut order = minimal_order();
        order.buyer_customer_party.party = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R010"));
    }

    #[test]
    fn test_missing_seller_party_fails() {
        let mut order = minimal_order();
        order.seller_supplier_party.party = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R012"));
    }

    #[test]
    fn test_missing_buyer_id_fails() {
        let mut order = minimal_order();
        order
            .buyer_customer_party
            .party
            .as_mut()
            .unwrap()
            .party_identification
            .clear();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R011"));
    }

    #[test]
    fn test_missing_seller_id_fails() {
        let mut order = minimal_order();
        order
            .seller_supplier_party
            .party
            .as_mut()
            .unwrap()
            .party_identification
            .clear();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R013"));
    }

    #[test]
    fn test_missing_country_warns() {
        let mut order = minimal_order();
        order
            .buyer_customer_party
            .party
            .as_mut()
            .unwrap()
            .postal_address
            .as_mut()
            .unwrap()
            .country = None;
        order
            .seller_supplier_party
            .party
            .as_mut()
            .unwrap()
            .postal_address
            .as_mut()
            .unwrap()
            .country = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R014"));
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R015"));
    }
}
