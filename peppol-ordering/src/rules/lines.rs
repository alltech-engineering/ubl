// Peppol BIS Ordering 3.0 — Order Line Business Rules
//
// Validates order line items for Purchase Orders.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Order>) {
    // ── ORD-R020 (Fatal): At least one order line must be present ─────────
    engine.add_rule(Rule {
        id: "ORD-R020".into(),
        description: "At least one order line must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.order_line.is_empty() {
                    Err("Order has no line items — at least one order line is required".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORD-R021 (Fatal): Each line must have a non-empty ID ──────────────
    engine.add_rule(Rule {
        id: "ORD-R021".into(),
        description: "Each line must have a non-empty ID".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if line.id.value().is_empty() {
                        return Err(format!("Order line {} has an empty ID", i + 1));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R022 (Fatal): Each line must have an item name ────────────────
    engine.add_rule(Rule {
        id: "ORD-R022".into(),
        description: "Each line must have an item name".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if line.item.name.is_none() {
                        return Err(format!(
                            "Order line {} item has no name",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R023 (Fatal): Each line must have an ordered quantity ─────────
    engine.add_rule(Rule {
        id: "ORD-R023".into(),
        description: "Each line must have an ordered quantity".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if line.quantity.is_none() {
                        return Err(format!(
                            "Order line {} is missing an ordered quantity",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R024 (Error): Each line should have a price ───────────────────
    engine.add_rule(Rule {
        id: "ORD-R024".into(),
        description: "Each line should have a price".into(),
        severity: Severity::Error,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if line.price.is_none() {
                        return Err(format!(
                            "Order line {} has no price specified",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R025 (Warning): Line note should be present for context ───────
    engine.add_rule(Rule {
        id: "ORD-R025".into(),
        description: "Line note should be present for context".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if line.note.is_empty() {
                        return Err(format!(
                            "Order line {} has no note — consider adding context",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════
    // NEW RULES (ORD-R027 through ORD-R033)
    // ═══════════════════════════════════════════════════════════════

    // ── ORD-R027 (Fatal): LineItem must have LineExtensionAmount (line total)
    engine.add_rule(Rule {
        id: "ORD-R027".into(),
        description: "LineItem must have LineExtensionAmount (line total)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if line.line_extension_amount.is_none() {
                        return Err(format!(
                            "Order line {} has no LineExtensionAmount — line total is required",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R028 (Error): Line total must equal quantity * price
    engine.add_rule(Rule {
        id: "ORD-R028".into(),
        description: "Line total must equal quantity * price".into(),
        severity: Severity::Error,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let (Some(qty), Some(line_ext), Some(price)) =
                        (&line.quantity, &line.line_extension_amount, &line.price)
                    {
                        let expected = qty.value * price.price_amount.value();
                        // Allow small rounding difference (tolerance of 0.01)
                        let diff = *line_ext.value() - expected;
                        if diff.abs().to_string().parse::<f64>().unwrap_or(1.0) > 0.02 {
                            return Err(format!(
                                "Order line {} line total {} does not match quantity {} * price {} = {}",
                                i + 1,
                                line_ext.value(),
                                qty.value,
                                price.price_amount.value(),
                                expected
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R029 (Fatal): Item identification (SellersItemIdentification) should be present
    engine.add_rule(Rule {
        id: "ORD-R029".into(),
        description: "Item identification (SellersItemIdentification) should be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if line.item.sellers_item_identification.is_none() {
                        return Err(format!(
                            "Order line {} has no SellersItemIdentification — item identification is required",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R030 (Warning): Item classification (commodity code) should be present
    engine.add_rule(Rule {
        id: "ORD-R030".into(),
        description: "Item classification (commodity code) should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if line.item.commodity_classification.is_empty() {
                        return Err(format!(
                            "Order line {} has no commodity classification — consider adding commodity code",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R031 (Fatal): Price amount must be present for each line
    engine.add_rule(Rule {
        id: "ORD-R031".into(),
        description: "Price amount must be present for each line".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    match &line.price {
                        None => {
                            return Err(format!(
                                "Order line {} has no price — price amount is required",
                                i + 1
                            ));
                        }
                        Some(price) => {
                            // price_amount is not optional on the Price struct, so it's always present
                            if *price.price_amount.value() == rust_decimal::Decimal::ZERO {
                                return Err(format!(
                                    "Order line {} has zero price amount — a positive price is required",
                                    i + 1
                                ));
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R032 (Fatal): BaseQuantity for price must match order quantity unit
    engine.add_rule(Rule {
        id: "ORD-R032".into(),
        description: "BaseQuantity for price must match order quantity unit".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let (Some(qty), Some(price)) =
                        (&line.quantity, &line.price)
                    {
                        if let Some(base_qty) = &price.base_quantity {
                            let qty_unit = qty.unit_code.as_deref().unwrap_or("");
                            let base_unit = base_qty.0.unit_code.as_deref().unwrap_or("");
                            if qty_unit != base_unit {
                                return Err(format!(
                                    "Order line {} price base quantity unit '{}' does not match order quantity unit '{}'",
                                    i + 1,
                                    base_unit,
                                    qty_unit
                                ));
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R033 (Warning): AllowanceCharge at line level should have reason code
    engine.add_rule(Rule {
        id: "ORD-R033".into(),
        description: "AllowanceCharge at line level should have reason code".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    for (j, ac) in line.allowance_charge.iter().enumerate() {
                        if ac.allowance_charge_reason_code.is_none() {
                            return Err(format!(
                                "Order line {} AllowanceCharge[{}] has no reason code — reason should be specified",
                                i + 1,
                                j + 1
                            ));
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
    fn test_no_lines_fails() {
        let mut order = minimal_order();
        order.order_line.clear();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R020"));
    }

    #[test]
    fn test_line_no_id_fails() {
        let mut order = minimal_order();
        order.order_line[0].id = cbc::ID::new("");
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R021"));
    }

    #[test]
    fn test_line_no_item_name_fails() {
        let mut order = minimal_order();
        order.order_line[0].item.name = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R022"));
    }

    #[test]
    fn test_line_no_quantity_fails() {
        let mut order = minimal_order();
        order.order_line[0].quantity = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R023"));
    }
}
