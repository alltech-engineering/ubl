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
                    Err(
                        "Order notes are present but all are empty — provide meaningful context"
                            .into(),
                    )
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

    // ── ORD-R007 (Fatal): OriginatorDocumentReference must reference prior order or contract if present
    engine.add_rule(Rule {
        id: "ORD-R007".into(),
        description: "OriginatorDocumentReference must reference a prior order or contract if present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(originator) = &inv.originator_document_reference {
                    match &originator.id {
                        None => Err("OriginatorDocumentReference is present but has no ID — must reference a prior order or contract".into()),
                        Some(id) if id.value().is_empty() => {
                            Err("OriginatorDocumentReference ID is empty — must reference a prior order or contract".into())
                        }
                        Some(_) => Ok(()),
                    }
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORD-R008 (Warning): QuotationDocumentReference should be present for traceability
    engine.add_rule(Rule {
        id: "ORD-R008".into(),
        description: "QuotationDocumentReference should be present for traceability".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.quotation_document_reference.is_none() {
                    Err(
                        "No quotation document reference — consider providing for traceability"
                            .into(),
                    )
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORD-R009 (Fatal): OrderDocumentReference is required if this is a change order
    engine.add_rule(Rule {
        id: "ORD-R009".into(),
        description: "OrderDocumentReference is required if this is a change order".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // Check if order type code indicates a change order (code "220" per UNCL 1001)
                let is_change_order = inv.order_type_code.as_ref()
                    .map(|c| c.value() == "230")
                    .unwrap_or(false);
                if is_change_order && inv.order_document_reference.is_empty() {
                    Err("Order is a change order but no OrderDocumentReference is provided — must reference the original order".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORD-R010 (Warning): AdditionalDocumentReference should have document type code
    engine.add_rule(Rule {
        id: "ORD-R010".into(),
        description: "AdditionalDocumentReference should have document type code".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, doc_ref) in inv.additional_document_reference.iter().enumerate() {
                    if doc_ref.document_type_code.is_none() {
                        return Err(format!(
                            "AdditionalDocumentReference[{}] has no DocumentTypeCode — type should be specified",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-R011 (Fatal): Contract reference ID must be present if contract is specified
    engine.add_rule(Rule {
        id: "ORD-R011".into(),
        description: "Contract reference ID must be present if contract is specified".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, contract) in inv.contract.iter().enumerate() {
                    match &contract.id {
                        None => {
                            return Err(format!(
                                "Contract[{}] has no ID — a contract reference ID is required",
                                i + 1
                            ));
                        }
                        Some(id) if id.value().is_empty() => {
                            return Err(format!(
                                "Contract[{}] ID is empty — a non-empty contract reference is required",
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

    // ── ORD-R012 (Warning): ValidityDuration should be specified for time-limited orders
    engine.add_rule(Rule {
        id: "ORD-R012".into(),
        description: "Validity period should be specified for time-limited orders".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.validity_period.is_empty() {
                    Err("Order has no validity period — consider specifying validity duration for time-limited orders".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORD-R013 (Warning): Issue date must not be in the future
    engine.add_rule(Rule {
        id: "ORD-R013".into(),
        description: "Issue date must not be in the future".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                let today = chrono::Utc::now().date_naive();
                if inv.issue_date.0 > today {
                    Err(format!(
                        "Issue date {} is in the future — document should not be post-dated",
                        inv.issue_date.0.format("%Y-%m-%d")
                    ))
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORD-R014 (Fatal): AccountingCostCode must be valid if present
    engine.add_rule(Rule {
        id: "ORD-R014".into(),
        description: "AccountingCostCode must be valid if present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.accounting_cost_code {
                None => Ok(()),
                Some(code) if code.value().is_empty() => {
                    Err("AccountingCostCode is present but empty — must be a valid code".into())
                }
                Some(_) => Ok(()),
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
            "validity_period": [{}],
            "note": [{"value": "Test order"}],
            "quotation_document_reference": {},
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
    fn test_valid_order_passes_all_header_rules() {
        let order = minimal_order();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(
            failures.is_empty(),
            "Expected no failures but got: {:?}",
            failures
        );
    }

    #[test]
    fn test_missing_order_id_fails() {
        let mut order = minimal_order();
        order.id = cbc::ID::new("");
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R001"));
    }

    #[test]
    fn test_missing_currency_fails() {
        let mut order = minimal_order();
        order.document_currency_code = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R003"));
    }

    #[test]
    fn test_missing_buyer_reference_warns() {
        let mut order = minimal_order();
        order.quotation_document_reference = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R006"));
    }
}
