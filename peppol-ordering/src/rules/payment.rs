// Peppol BIS Ordering 3.0 — Payment Business Rules
//
// Validates payment means and terms for Purchase Orders.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Order>) {
    // ── ORD-R040 (Warning): Payment means should be specified ─────────────
    engine.add_rule(Rule {
        id: "ORD-R040".into(),
        description: "Payment means should be specified".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.payment_means.is_empty() {
                    Err("No payment means specified — consider defining how payment should be made".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORD-R041 (Warning): Payment terms should be specified ─────────────
    engine.add_rule(Rule {
        id: "ORD-R041".into(),
        description: "Payment terms should be specified".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.payment_terms.is_empty() {
                    Err("No payment terms specified — consider defining payment timing and conditions".into())
                } else {
                    Ok(())
                }
            })
        },
    });
}
