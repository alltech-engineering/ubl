// Peppol BIS Ordering 3.0 — Code List Validation Rules
//
// Validates that coded values belong to the correct ISO / UNCL code lists.

use peppol_common::codes::{currency_codes, payment_means_codes};
use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Order>) {
    // ── ORD-CL001 (Fatal): DocumentCurrencyCode must be valid ISO 4217 ────
    engine.add_rule(Rule {
        id: "ORD-CL001".into(),
        description: "DocumentCurrencyCode must be a valid ISO 4217 currency code".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.document_currency_code {
                None => Err("DocumentCurrencyCode is missing — required for Peppol BIS".into()),
                Some(cc) => {
                    let code = cc.value();
                    if currency_codes().is_valid(code) {
                        Ok(())
                    } else {
                        Err(format!(
                            "DocumentCurrencyCode '{}' is not a valid ISO 4217 currency code",
                            code
                        ))
                    }
                }
            })
        },
    });

    // ── ORD-CL002 (Fatal): PaymentMeansCode must be from UNCL 4461 ────────
    engine.add_rule(Rule {
        id: "ORD-CL002".into(),
        description: "PaymentMeansCode must be a valid UNCL 4461 code".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, pm) in inv.payment_means.iter().enumerate() {
                    let code = pm.payment_means_code.value();
                    if !payment_means_codes().is_valid(code) {
                        return Err(format!(
                            "PaymentMeans[{}] code '{}' is not a valid UNCL4461 code",
                            i + 1,
                            code
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════
    // NEW RULES (ORD-CL003 through ORD-CL007)
    // ═══════════════════════════════════════════════════════════════

    // ── ORD-CL003 (Fatal): PricingCurrencyCode must be valid ISO 4217 if present
    engine.add_rule(Rule {
        id: "ORD-CL003".into(),
        description: "PricingCurrencyCode must be a valid ISO 4217 currency code if present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.pricing_currency_code {
                None => Ok(()),
                Some(cc) => {
                    let code = cc.value();
                    if code.is_empty() {
                        Err("PricingCurrencyCode is present but empty".into())
                    } else if currency_codes().is_valid(code) {
                        Ok(())
                    } else {
                        Err(format!(
                            "PricingCurrencyCode '{}' is not a valid ISO 4217 currency code",
                            code
                        ))
                    }
                }
            })
        },
    });

    // ── ORD-CL004 (Fatal): TaxCurrencyCode must be valid ISO 4217 if present
    engine.add_rule(Rule {
        id: "ORD-CL004".into(),
        description: "TaxCurrencyCode must be a valid ISO 4217 currency code if present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.tax_currency_code {
                None => Ok(()),
                Some(cc) => {
                    let code = cc.value();
                    if code.is_empty() {
                        Err("TaxCurrencyCode is present but empty".into())
                    } else if currency_codes().is_valid(code) {
                        Ok(())
                    } else {
                        Err(format!(
                            "TaxCurrencyCode '{}' is not a valid ISO 4217 currency code",
                            code
                        ))
                    }
                }
            })
        },
    });

    // ── ORD-CL005 (Fatal): Country codes must be valid ISO 3166-1 alpha-2
    engine.add_rule(Rule {
        id: "ORD-CL005".into(),
        description: "Country codes must be valid ISO 3166-1 alpha-2".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // Validate destination country if present
                if let Some(dest_country) = &inv.destination_country {
                    if let Some(code) = &dest_country.identification_code {
                        let v = code.value();
                        if v.len() != 2 || !v.chars().all(|c| c.is_ascii_uppercase()) {
                            return Err(format!(
                                "Destination country code '{}' is not a valid ISO 3166-1 alpha-2 code",
                                v
                            ));
                        }
                    }
                }

                // Validate buyer country if present
                if let Some(party) = &inv.buyer_customer_party.party {
                    if let Some(addr) = &party.postal_address {
                        if let Some(country) = &addr.country {
                            if let Some(code) = &country.identification_code {
                                let v = code.value();
                                if v.len() != 2 || !v.chars().all(|c| c.is_ascii_uppercase()) {
                                    return Err(format!(
                                        "Buyer country code '{}' is not a valid ISO 3166-1 alpha-2 code",
                                        v
                                    ));
                                }
                            }
                        }
                    }
                }

                // Validate seller country if present
                if let Some(party) = &inv.seller_supplier_party.party {
                    if let Some(addr) = &party.postal_address {
                        if let Some(country) = &addr.country {
                            if let Some(code) = &country.identification_code {
                                let v = code.value();
                                if v.len() != 2 || !v.chars().all(|c| c.is_ascii_uppercase()) {
                                    return Err(format!(
                                        "Seller country code '{}' is not a valid ISO 3166-1 alpha-2 code",
                                        v
                                    ));
                                }
                            }
                        }
                    }
                }

                Ok(())
            })
        },
    });

    // ── ORD-CL006 (Warning): Unit codes in quantities should be from UN/ECE Rec.20
    engine.add_rule(Rule {
        id: "ORD-CL006".into(),
        description: "Unit codes in quantities should be from UN/ECE Rec.20".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.order_line.iter().enumerate() {
                    if let Some(ref li) = line.line_item {
                        if let Some(ref qty) = li.quantity {
                            match &qty.unit_code {
                                None => {
                                    return Err(format!(
                                        "Order line {} quantity has no unit code — unit should be specified from UN/ECE Rec.20",
                                        i + 1
                                    ));
                                }
                                Some(unit) if unit.is_empty() => {
                                    return Err(format!(
                                        "Order line {} quantity has an empty unit code — unit should be from UN/ECE Rec.20",
                                        i + 1
                                    ));
                                }
                                Some(_) => {}
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── ORD-CL007 (Fatal): DocumentTypeCode on AdditionalDocumentReference must be valid
    engine.add_rule(Rule {
        id: "ORD-CL007".into(),
        description: "DocumentTypeCode on AdditionalDocumentReference must be valid (non-empty)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, doc_ref) in inv.additional_document_reference.iter().enumerate() {
                    match &doc_ref.document_type_code {
                        None => {
                            return Err(format!(
                                "AdditionalDocumentReference[{}] has no DocumentTypeCode — a valid type code is required",
                                i + 1
                            ));
                        }
                        Some(code) if code.value().is_empty() => {
                            return Err(format!(
                                "AdditionalDocumentReference[{}] DocumentTypeCode is empty — must be a valid UNCL 1001 code",
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
    fn test_invalid_currency_fails() {
        let mut order = minimal_order();
        order.document_currency_code = Some(cbc::DocumentCurrencyCode::new("XXX"));
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-CL001"));
    }
}
