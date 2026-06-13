// Peppol BIS Ordering 3.0 — Delivery Business Rules
//
// Validates delivery information for Purchase Orders.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Order>) {
    // ── ORD-R030 (Warning): Delivery information should be present ────────
    engine.add_rule(Rule {
        id: "ORD-R030".into(),
        description: "Delivery information should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.delivery.is_empty() {
                    Err("No delivery information provided — consider specifying delivery details".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORD-R031 (Warning): Delivery location address should be present ───
    engine.add_rule(Rule {
        id: "ORD-R031".into(),
        description: "Delivery location address should be present if delivery is specified".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, delivery) in inv.delivery.iter().enumerate() {
                    if delivery.delivery_address.is_none() {
                        return Err(format!(
                            "Delivery[{}] has no delivery address — location should be specified",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R032 (Warning): Requested delivery period should be specified ─
    engine.add_rule(Rule {
        id: "ORD-R032".into(),
        description: "Requested delivery period should be specified".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, delivery) in inv.delivery.iter().enumerate() {
                    if delivery.requested_delivery_period.is_none() {
                        return Err(format!(
                            "Delivery[{}] has no requested delivery period — expected delivery date should be specified",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════
    // NEW RULES (ORD-R034 through ORD-R039)
    // ═══════════════════════════════════════════════════════════════

    // ── ORD-R034 (Fatal): Delivery location country code must be present if delivery is specified
    engine.add_rule(Rule {
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
    });

    // ── ORD-R035 (Warning): Delivery party should be specified for ship-to scenarios
    engine.add_rule(Rule {
        id: "ORD-R035".into(),
        description: "Delivery party should be specified for ship-to scenarios".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, delivery) in inv.delivery.iter().enumerate() {
                    if delivery.delivery_party.is_none() {
                        return Err(format!(
                            "Delivery[{}] has no delivery party — should be specified for ship-to scenarios",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R036 (Fatal): RequestedDeliveryPeriod start/end must be valid dates
    engine.add_rule(Rule {
        id: "ORD-R036".into(),
        description: "RequestedDeliveryPeriod start/end must be valid dates".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, delivery) in inv.delivery.iter().enumerate() {
                    if let Some(period) = &delivery.requested_delivery_period {
                        if let (Some(start), Some(end)) = (&period.start_date, &period.end_date) {
                            if start.0 > end.0 {
                                return Err(format!(
                                    "Delivery[{}] requested period start {} is after end {}",
                                    i + 1,
                                    start.0.format("%Y-%m-%d"),
                                    end.0.format("%Y-%m-%d")
                                ));
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R037 (Warning): DeliveryTerms should include special instructions
    engine.add_rule(Rule {
        id: "ORD-R037".into(),
        description: "DeliveryTerms should include special instructions".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, dt) in inv.delivery_terms.iter().enumerate() {
                    if dt.special_terms.is_empty() {
                        return Err(format!(
                            "DeliveryTerms[{}] has no special terms — consider including delivery instructions",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R038 (Warning): Shipment information should include transport details
    engine.add_rule(Rule {
        id: "ORD-R038".into(),
        description: "Shipment information should include transport details".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, delivery) in inv.delivery.iter().enumerate() {
                    if let Some(shipment) = &delivery.shipment {
                        if shipment.information.is_empty() && shipment.goods_item.is_empty() {
                            return Err(format!(
                                "Delivery[{}] shipment has no transport details — consider adding information or goods items",
                                i + 1
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R039 (Fatal): Delivery address country must be ISO 3166-1 alpha-2
    engine.add_rule(Rule {
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
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use peppol_common::rules::RuleEngine;
    use std::sync::Arc;
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
    fn test_no_delivery_warns() {
        let order = minimal_order();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R030"));
    }
}
