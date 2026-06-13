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

    // ═══════════════════════════════════════════════════════════════
    // NEW RULES (ORD-R042 through ORD-R045)
    // ═══════════════════════════════════════════════════════════════

    // ── ORD-R042 (Fatal): PaymentMeans payment_means_code must be valid
    engine.add_rule(Rule {
        id: "ORD-R042".into(),
        description: "PaymentMeans payment_means_code must be valid (non-empty)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, pm) in inv.payment_means.iter().enumerate() {
                    let code = pm.payment_means_code.value();
                    if code.is_empty() {
                        return Err(format!(
                            "PaymentMeans[{}] has an empty payment_means_code — a valid code is required",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R043 (Warning): PaymentMeans should include financial account details
    engine.add_rule(Rule {
        id: "ORD-R043".into(),
        description: "PaymentMeans should include financial account details".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, pm) in inv.payment_means.iter().enumerate() {
                    if pm.payee_financial_account.is_none()
                        && pm.payer_financial_account.is_none()
                    {
                        return Err(format!(
                            "PaymentMeans[{}] has no financial account details — consider providing account information",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R044 (Warning): PaymentTerms should include settlement period
    engine.add_rule(Rule {
        id: "ORD-R044".into(),
        description: "PaymentTerms should include settlement period".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, pt) in inv.payment_terms.iter().enumerate() {
                    if pt.settlement_period.is_none() {
                        return Err(format!(
                            "PaymentTerms[{}] has no settlement period — consider specifying when payment is due",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R045 (Warning): PaymentTerms note for discount/penalty information
    engine.add_rule(Rule {
        id: "ORD-R045".into(),
        description: "PaymentTerms should include discount/penalty information in notes".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, pt) in inv.payment_terms.iter().enumerate() {
                    if pt.note.is_empty() {
                        return Err(format!(
                            "PaymentTerms[{}] has no notes — consider specifying discount or penalty information",
                            i + 1
                        ));
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
    fn test_no_payment_warns() {
        let order = minimal_order();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R040"));
    }
}
