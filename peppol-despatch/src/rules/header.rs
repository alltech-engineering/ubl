// Peppol BIS Despatch Advice 3.0 — Document Header Rules
//
// Validates document-level metadata for Despatch Advice.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use rust_decimal::prelude::ToPrimitive;
use std::sync::Arc;
use ubl_documents::despatch::DespatchAdvice;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<DespatchAdvice>) {
    // ── DESP-R001 (Fatal): OrderReference must be present ───────────────
    engine.add_rule(Rule {
        id: "DESP-R001".into(),
        description: "OrderReference must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.order_reference.is_empty() {
                    Err("OrderReference is missing — at least one order reference is required".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── DESP-R002 (Fatal): ID must be present and non-empty ─────────────
    engine.add_rule(Rule {
        id: "DESP-R002".into(),
        description: "DespatchAdvice ID must be present and non-empty".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.id.value().is_empty() {
                    Err("DespatchAdvice ID is empty — a non-empty identifier is required".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── DESP-R003 (Fatal): DespatchAdviceTypeCode must be present ───────
    engine.add_rule(Rule {
        id: "DESP-R003".into(),
        description: "DespatchAdviceTypeCode must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.despatch_advice_type_code {
                None => Err("DespatchAdviceTypeCode is missing — required for Peppol BIS".into()),
                Some(tc) if tc.value().is_empty() => {
                    Err("DespatchAdviceTypeCode is present but empty".into())
                }
                Some(_) => Ok(()),
            })
        },
    });

    // ── DESP-R004 (Fatal): DocumentStatusCode must be valid if present ──
    engine.add_rule(Rule {
        id: "DESP-R004".into(),
        description: "DocumentStatusCode must be non-empty if present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.document_status_code {
                None => Ok(()),
                Some(dsc) if dsc.value().is_empty() => {
                    Err("DocumentStatusCode is present but empty".into())
                }
                Some(_) => Ok(()),
            })
        },
    });

    // ── DESP-R005 (Warning): UBLVersionID should be present ─────────────
    engine.add_rule(Rule {
        id: "DESP-R005".into(),
        description: "UBLVersionID should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.ubl_version_id {
                None => Err("UBLVersionID is missing — should be '2.2' for Peppol BIS".into()),
                Some(v) if v.value().is_empty() => {
                    Err("UBLVersionID is present but empty".into())
                }
                Some(_) => Ok(()),
            })
        },
    });

    // ── DESP-R006 (Warning): CustomizationID should be present ──────────
    engine.add_rule(Rule {
        id: "DESP-R006".into(),
        description: "CustomizationID should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.customization_id {
                None => Err(
                    "CustomizationID is missing — should identify the Peppol BIS customization"
                        .into(),
                ),
                Some(c) if c.value().is_empty() => {
                    Err("CustomizationID is present but empty".into())
                }
                Some(_) => Ok(()),
            })
        },
    });

    // ── DESP-R007 (Warning): ProfileID should be present ────────────────
    engine.add_rule(Rule {
        id: "DESP-R007".into(),
        description: "ProfileID should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.profile_id {
                None => Err("ProfileID is missing — should identify the Peppol BIS process".into()),
                Some(p) if p.value().is_empty() => {
                    Err("ProfileID is present but empty".into())
                }
                Some(_) => Ok(()),
            })
        },
    });

    // ── DESP-R008 (Fatal): Each OrderReference must have a non-empty ID ─
    engine.add_rule(Rule {
        id: "DESP-R008".into(),
        description: "Each OrderReference must have a non-empty ID".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, order_ref) in inv.order_reference.iter().enumerate() {
                    match &order_ref.id {
                        None => {
                            return Err(format!(
                                "OrderReference {} has no ID — an order reference identifier is required",
                                i + 1
                            ));
                        }
                        Some(id) if id.value().is_empty() => {
                            return Err(format!(
                                "OrderReference {} has an empty ID",
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

    // ── DESP-R009 (Warning): LineCountNumeric should match actual lines ─
    engine.add_rule(Rule {
        id: "DESP-R009".into(),
        description: "LineCountNumeric should match the number of despatch lines".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.line_count_numeric {
                None => Ok(()),
                Some(lcn) => {
                    let expected = inv.despatch_line.len() as u32;
                    let actual = lcn.0.to_u32().unwrap_or(0);
                    if actual != expected {
                        Err(format!(
                            "LineCountNumeric ({}) does not match actual despatch line count ({})",
                            actual, expected
                        ))
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ── DESP-R010 (Fatal): At least one AdditionalDocumentReference ─────
    engine.add_rule(Rule {
        id: "DESP-R010".into(),
        description: "At least one AdditionalDocumentReference should be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.additional_document_reference.is_empty() {
                    Err("AdditionalDocumentReference is missing — at least one supporting document reference is required".into())
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
    use ubl_documents::despatch::DespatchAdvice;

    fn minimal_despatch() -> DespatchAdvice {
        serde_json::from_str(
            r#"{
            "id": {"value": "DES-001"},
            "issue_date": "2026-06-13",
            "ubl_version_id": {"value": "2.2"},
            "customization_id": {"value": "urn:cen.eu:en16931:2017#compliant#urn:fdc:peppol.eu:2017:poacc:billing:3.0"},
            "profile_id": {"value": "urn:fdc:peppol.eu:2017:poacc:billing:01:1.0"},
            "despatch_advice_type_code": {"value": "delivery"},
            "order_reference": [{"id": {"value": "ORD-001"}}],
            "additional_document_reference": [{
                "id": {"value": "DOC-001"},
                "document_type_code": {"value": "130"}
            }],
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
    fn test_valid_despatch_passes() {
        let despatch = minimal_despatch();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(despatch));
        let failures = engine.evaluate_failures();
        assert!(failures.is_empty(), "Expected no failures but got: {:?}", failures);
    }

    #[test]
    fn test_missing_order_ref_fails() {
        let mut despatch = minimal_despatch();
        despatch.order_reference.clear();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(despatch));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "DESP-R001"));
    }

    #[test]
    fn test_missing_id_fails() {
        let mut despatch = minimal_despatch();
        despatch.id = cbc::ID::new("");
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(despatch));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "DESP-R002"));
    }
}
