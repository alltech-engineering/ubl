// Peppol BIS Despatch Advice 3.0 — Despatch Line Business Rules
//
// Validates despatch line items for Despatch Advice.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::despatch::DespatchAdvice;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<DespatchAdvice>) {
    // ── DESP-R023 (Fatal): At least one despatch line must be present ────
    engine.add_rule(Rule {
        id: "DESP-R023".into(),
        description: "At least one despatch line must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.despatch_line.is_empty() {
                    Err("DespatchAdvice has no line items — at least one despatch line is required".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── DESP-R024 (Fatal): Each line item must have a non-empty name ─────
    engine.add_rule(Rule {
        id: "DESP-R024".into(),
        description: "Each despatch line item must have a non-empty name".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.despatch_line.iter().enumerate() {
                    match &line.item.name {
                        None => {
                            return Err(format!(
                                "Despatch line {} item has no name",
                                i + 1
                            ));
                        }
                        Some(name) if name.value().is_empty() => {
                            return Err(format!(
                                "Despatch line {} item has an empty name",
                                i + 1
                            ));
                        }
                        Some(_) => {}
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R025 (Fatal): Each line must have a delivered quantity ──────
    engine.add_rule(Rule {
        id: "DESP-R025".into(),
        description: "Each despatch line must have a delivered quantity".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.despatch_line.iter().enumerate() {
                    if line.delivered_quantity.is_none() {
                        return Err(format!(
                            "Despatch line {} is missing a delivered quantity",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R026 (Fatal): Each line ID must be non-empty ────────────────
    engine.add_rule(Rule {
        id: "DESP-R026".into(),
        description: "Each despatch line ID must be non-empty".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.despatch_line.iter().enumerate() {
                    if line.id.value().is_empty() {
                        return Err(format!(
                            "Despatch line {} has an empty ID",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R027 (Warning): Each line should have an OrderLineReference ─
    engine.add_rule(Rule {
        id: "DESP-R027".into(),
        description: "Each despatch line should reference an order line".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.despatch_line.iter().enumerate() {
                    if line.order_line_reference.is_empty() {
                        return Err(format!(
                            "Despatch line {} has no OrderLineReference",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R028 (Fatal): OrderLineReference LineID must be non-empty ───
    engine.add_rule(Rule {
        id: "DESP-R028".into(),
        description: "Each OrderLineReference must have a non-empty LineID".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.despatch_line.iter().enumerate() {
                    for (j, olr) in line.order_line_reference.iter().enumerate() {
                        if olr.line_id.value().is_empty() {
                            return Err(format!(
                                "Despatch line {}, OrderLineReference {} has an empty LineID",
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

    // ── DESP-R029 (Fatal): Delivered quantity must be positive ───────────
    engine.add_rule(Rule {
        id: "DESP-R029".into(),
        description: "Delivered quantity must be greater than zero".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.despatch_line.iter().enumerate() {
                    if let Some(ref qty) = line.delivered_quantity {
                        let val = qty.value();
                        if val.is_zero() || val.is_sign_negative() {
                            return Err(format!(
                                "Despatch line {} has a non-positive delivered quantity ({})",
                                i + 1,
                                val
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R030 (Fatal): Each line item name must be present ───────────
    // (Already covered by DESP-R024 with the type-enforced Item)
    // Kept as a separate rule for spec compliance.
    engine.add_rule(Rule {
        id: "DESP-R030".into(),
        description: "Each line item must have a non-empty name".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.despatch_line.iter().enumerate() {
                    let item = &line.item;
                    match &item.name {
                        None => {
                            return Err(format!(
                                "Despatch line {} item has no name",
                                i + 1
                            ));
                        }
                        Some(name) if name.value().is_empty() => {
                            return Err(format!(
                                "Despatch line {} item has an empty name",
                                i + 1
                            ));
                        }
                        Some(_) => {}
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R031 (Warning): LineStatusCode should be valid if present ───
    engine.add_rule(Rule {
        id: "DESP-R031".into(),
        description: "LineStatusCode must be a valid code if present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.despatch_line.iter().enumerate() {
                    if let Some(ref status) = line.line_status_code {
                        if status.value().is_empty() {
                            return Err(format!(
                                "Despatch line {} has an empty LineStatusCode",
                                i + 1
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R033 (Warning): Outstanding quantity should not be negative ──
    engine.add_rule(Rule {
        id: "DESP-R033".into(),
        description: "Outstanding quantity should not be negative".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.despatch_line.iter().enumerate() {
                    if let Some(ref qty) = line.outstanding_quantity {
                        if qty.value.is_sign_negative() {
                            return Err(format!(
                                "Despatch line {} has a negative outstanding quantity ({})",
                                i + 1,
                                qty.value
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
    fn test_no_lines_fails() {
        let mut despatch = minimal_despatch();
        despatch.despatch_line.clear();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(despatch));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "DESP-R023"));
    }

    #[test]
    fn test_line_no_item_name_fails() {
        let mut despatch = minimal_despatch();
        despatch.despatch_line[0].item.name = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(despatch));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "DESP-R024"));
    }

    #[test]
    fn test_line_no_quantity_fails() {
        let mut despatch = minimal_despatch();
        despatch.despatch_line[0].delivered_quantity = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(despatch));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "DESP-R025"));
    }
}
