// Peppol BIS Catalogue 3.0 — Document Header Rules
//
// Validates document-level metadata for Catalogues.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::catalogue::Catalogue;

pub fn add_rules(engine: &mut RuleEngine, cat: &Arc<Catalogue>) {
    // ── CAT-R001 (Fatal): Catalogue ID must be present ──────────────────
    engine.add_rule(Rule {
        id: "CAT-R001".into(),
        description: "Catalogue ID must be present and non-empty".into(),
        severity: Severity::Fatal,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                if cat.id.value().is_empty() {
                    Err("Catalogue ID is empty — a non-empty catalogue identifier is required"
                        .into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── CAT-R002 (Fatal): Issue date must be present ────────────────────
    engine.add_rule(Rule {
        id: "CAT-R002".into(),
        description: "Issue date must be present".into(),
        severity: Severity::Fatal,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                let _date = &cat.issue_date;
                Ok(())
            })
        },
    });

    // ── CAT-R003 (Fatal): Provider party (seller) must be present with identification
    engine.add_rule(Rule {
        id: "CAT-R003".into(),
        description: "Provider party (seller) must be present with identification".into(),
        severity: Severity::Fatal,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                if cat.provider_party.party_identification.is_empty() {
                    Err(
                        "Provider party has no identification — a Peppol participant ID is required"
                            .into(),
                    )
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── CAT-R004 (Fatal): Receiver party (buyer) must be present with identification
    engine.add_rule(Rule {
        id: "CAT-R004".into(),
        description: "Receiver party (buyer) must be present with identification".into(),
        severity: Severity::Fatal,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                if cat.receiver_party.party_identification.is_empty() {
                    Err(
                        "Receiver party has no identification — a Peppol participant ID is required"
                            .into(),
                    )
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── CAT-R005 (Warning): Validity period should be specified ──────────
    engine.add_rule(Rule {
        id: "CAT-R005".into(),
        description: "Validity period should be specified".into(),
        severity: Severity::Warning,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                if cat.validity_period.is_empty() {
                    Err(
                        "Catalogue has no validity period — consider specifying how long this catalogue is valid"
                            .into(),
                    )
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
    use ubl_documents::catalogue::Catalogue;

    fn minimal_catalogue() -> Catalogue {
        serde_json::from_str(
            r#"{
            "id": {"value": "CAT-001"},
            "issue_date": "2026-06-13",
            "provider_party": {
                "party_name": [{"name": "Supplier Corp"}],
                "party_identification": [{"id": {"value": "9933:supplier456"}}]
            },
            "receiver_party": {
                "party_name": [{"name": "Buyer Ltd"}],
                "party_identification": [{"id": {"value": "9933:buyer123"}}]
            },
            "validity_period": [{}],
            "catalogue_line": [{
                "id": {"value": "1"},
                "orderable_indicator": true,
                "item": {
                    "name": "Widget",
                    "sellers_item_identification": {"id": {"value": "WID-001"}},
                    "commodity_classification": [{"item_classification_code": {"value": "12345"}}]
                },
                "price": {
                    "price_amount": {"value": "100.00", "currency_id": "ZAR"}
                }
            }]
        }"#,
        )
        .unwrap()
    }

    #[test]
    fn test_valid_catalogue_passes_all_header_rules() {
        let catalogue = minimal_catalogue();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(catalogue));
        let failures = engine.evaluate_failures();
        assert!(
            failures.is_empty(),
            "Expected no header failures but got: {:?}",
            failures
        );
    }

    #[test]
    fn test_missing_catalogue_id_fails() {
        let mut catalogue = minimal_catalogue();
        catalogue.id = cbc::ID::new("");
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(catalogue));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "CAT-R001"));
    }

    #[test]
    fn test_missing_provider_party_identification_fails() {
        let mut catalogue = minimal_catalogue();
        catalogue.provider_party.party_identification.clear();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(catalogue));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "CAT-R003"));
    }
}
