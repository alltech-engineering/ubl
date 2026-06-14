// Peppol BIS Catalogue 3.0 — Catalogue Line Business Rules
//
// Validates catalogue line items for Catalogues.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::catalogue::Catalogue;

pub fn add_rules(engine: &mut RuleEngine, cat: &Arc<Catalogue>) {
    // ── CAT-R010 (Fatal): At least one catalogue line must be present ────
    engine.add_rule(Rule {
        id: "CAT-R010".into(),
        description: "At least one catalogue line must be present".into(),
        severity: Severity::Fatal,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                if cat.catalogue_line.is_empty() {
                    Err("Catalogue has no line items — at least one catalogue line is required"
                        .into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── CAT-R011 (Fatal): Each line ID must be present ───────────────────
    engine.add_rule(Rule {
        id: "CAT-R011".into(),
        description: "Each line must have a non-empty ID".into(),
        severity: Severity::Fatal,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                for (i, line) in cat.catalogue_line.iter().enumerate() {
                    if line.id.value().is_empty() {
                        return Err(format!("Catalogue line {} has an empty ID", i + 1));
                    }
                }
                Ok(())
            })
        },
    });

    // ── CAT-R012 (Fatal): Each line item name must be present ────────────
    engine.add_rule(Rule {
        id: "CAT-R012".into(),
        description: "Each line must have an item name".into(),
        severity: Severity::Fatal,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                for (i, line) in cat.catalogue_line.iter().enumerate() {
                    match &line.item.name {
                        None => {
                            return Err(format!(
                                "Catalogue line {} item has no name",
                                i + 1
                            ));
                        }
                        Some(name) if name.value().trim().is_empty() => {
                            return Err(format!(
                                "Catalogue line {} item name is empty",
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

    // ── CAT-R013 (Fatal): Item identification (SellersItemIdentification) must be present
    engine.add_rule(Rule {
        id: "CAT-R013".into(),
        description:
            "Item identification (SellersItemIdentification) should be present".into(),
        severity: Severity::Fatal,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                for (i, line) in cat.catalogue_line.iter().enumerate() {
                    if line.item.sellers_item_identification.is_none() {
                        return Err(format!(
                            "Catalogue line {} has no SellersItemIdentification — item identification is required",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── CAT-R014 (Warning): Item classification (commodity code) should be present
    engine.add_rule(Rule {
        id: "CAT-R014".into(),
        description: "Item classification (commodity code) should be present".into(),
        severity: Severity::Warning,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                for (i, line) in cat.catalogue_line.iter().enumerate() {
                    if line.item.commodity_classification.is_empty() {
                        return Err(format!(
                            "Catalogue line {} has no commodity classification — consider adding commodity code",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── CAT-R015 (Fatal): Price must be present ──────────────────────────
    engine.add_rule(Rule {
        id: "CAT-R015".into(),
        description: "Price must be present for each line".into(),
        severity: Severity::Fatal,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                for (i, line) in cat.catalogue_line.iter().enumerate() {
                    match &line.price {
                        None => {
                            return Err(format!(
                                "Catalogue line {} has no price — price is required",
                                i + 1
                            ));
                        }
                        Some(price) => {
                            if *price.price_amount.value() == rust_decimal::Decimal::ZERO {
                                return Err(format!(
                                    "Catalogue line {} has zero price amount — a positive price is required",
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

    // ── CAT-R016 (Warning): Item description should be present ───────────
    engine.add_rule(Rule {
        id: "CAT-R016".into(),
        description: "Item description should be present".into(),
        severity: Severity::Warning,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                for (i, line) in cat.catalogue_line.iter().enumerate() {
                    if line.item.description.is_none() {
                        return Err(format!(
                            "Catalogue line {} has no item description — consider adding description",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── CAT-R017 (Fatal): Orderable indicator should be present ──────────
    engine.add_rule(Rule {
        id: "CAT-R017".into(),
        description: "Orderable indicator should be present".into(),
        severity: Severity::Fatal,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                for (i, line) in cat.catalogue_line.iter().enumerate() {
                    if line.orderable_indicator.is_none() {
                        return Err(format!(
                            "Catalogue line {} has no orderable indicator — must specify whether item is orderable",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── CAT-R018 (Warning): Unit code for orderable unit should be valid ─
    engine.add_rule(Rule {
        id: "CAT-R018".into(),
        description: "Unit code for orderable unit should be valid".into(),
        severity: Severity::Warning,
        check: {
            let cat = Arc::clone(cat);
            Box::new(move || {
                for (i, line) in cat.catalogue_line.iter().enumerate() {
                    if let Some(orderable_unit) = &line.orderable_unit {
                        if orderable_unit.value().trim().is_empty() {
                            return Err(format!(
                                "Catalogue line {} has an empty orderable unit code",
                                i + 1
                            ));
                        }
                    } else {
                        return Err(format!(
                            "Catalogue line {} has no orderable unit specified — consider adding unit code",
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
    fn test_no_lines_fails() {
        let mut catalogue = minimal_catalogue();
        catalogue.catalogue_line.clear();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(catalogue));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "CAT-R010"));
    }

    #[test]
    fn test_line_no_id_fails() {
        let mut catalogue = minimal_catalogue();
        catalogue.catalogue_line[0].id = cbc::ID::new("");
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(catalogue));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "CAT-R011"));
    }

    #[test]
    fn test_line_no_item_name_fails() {
        let mut catalogue = minimal_catalogue();
        catalogue.catalogue_line[0].item.name = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(catalogue));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "CAT-R012"));
    }
}
