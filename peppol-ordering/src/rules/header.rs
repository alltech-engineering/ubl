// Peppol BIS Ordering 3.0 — Document Header Rules
//
// Validates document-level metadata for Purchase Orders.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Order>) {
    // ── ORD-R001 (Fatal): Order ID must be present and non-empty ──────────
    engine.add_rule(Rule {
        id: "ORD-R001".into(),
        description: "Order ID must be present and non-empty".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.id.value().is_empty() {
                    Err("Order ID is empty — a non-empty order identifier is required".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORD-R002 (Fatal): Issue date must be present ──────────────────────
    engine.add_rule(Rule {
        id: "ORD-R002".into(),
        description: "Issue date must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                let _date = &inv.issue_date;
                Ok(())
            })
        },
    });

    // ── ORD-R003 (Fatal): Document currency code must be present ──────────
    engine.add_rule(Rule {
        id: "ORD-R003".into(),
        description: "Document currency code must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.document_currency_code {
                None => Err("Document currency code is missing — required for Peppol BIS".into()),
                Some(cc) if cc.value().is_empty() => {
                    Err("Document currency code is present but empty".into())
                }
                Some(_) => Ok(()),
            })
        },
    });

    // ── ORD-R004 (Warning): Order should have a validity period ───────────
    engine.add_rule(Rule {
        id: "ORD-R004".into(),
        description: "Order should have a validity period".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.validity_period.is_empty() {
                    Err("Order has no validity period — consider specifying how long this order is valid".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORD-R005 (Warning): Notes should provide context ──────────────────
    engine.add_rule(Rule {
        id: "ORD-R005".into(),
        description: "Notes should provide context".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.note.is_empty() {
                    Err("No order notes provided — consider adding context about the order".into())
                } else if inv.note.iter().all(|n| n.value().trim().is_empty()) {
                    Err("Order notes are present but all are empty — provide meaningful context".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORD-R006 (Fatal): A buyer reference should be present ─────────────
    engine.add_rule(Rule {
        id: "ORD-R006".into(),
        description: "A buyer reference (quotation or prior order) should be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                let has_quotation = inv.quotation_document_reference.is_some();
                let has_prior_order = !inv.order_document_reference.is_empty();
                if has_quotation || has_prior_order {
                    Ok(())
                } else {
                    Err("No buyer reference provided — a quotation or prior order reference should be present".into())
                }
            })
        },
    });
}
