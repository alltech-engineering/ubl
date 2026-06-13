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
}
