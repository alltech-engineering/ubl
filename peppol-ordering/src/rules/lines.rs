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
}
