// Peppol BIS Billing 3.0 — Document Identity & Header Rules
//
// Validates document-level metadata per EN16931/Peppol BIS Billing 3.0.
// Rules reference: EN16931 Schematron and Peppol BIS Billing 3.0 specification.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::billing::Invoice;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Invoice>) {
    // ── R001: ProfileID must be present ──────────────────────────────────
    // Informational only — the DocumentIdentity wrapper enforces this at a
    // higher level. We emit a warning if missing so users can see it.
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R001".into(),
        description: "ProfileID must be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.profile_id.is_none() {
                    Err("ProfileID is missing — document must declare a Peppol profile".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── R004: CustomizationID must contain EN16931 + Peppol URN ─────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R004".into(),
        description: "CustomizationID must contain EN16931 and Peppol Billing URNs".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.customization_id {
                None => Err(
                    "CustomizationID is missing — required for Peppol BIS Billing 3.0".into(),
                ),
                Some(cid) => {
                    let v = cid.value().to_lowercase();
                    if v.contains("en16931") && v.contains("peppol") {
                        Ok(())
                    } else {
                        Err(format!(
                            "CustomizationID '{}' must contain both EN16931 and Peppol URN fragments",
                            cid.value()
                        ))
                    }
                }
            })
        },
    });

    // ── BT-001: Invoice ID must be present and non-empty ─────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT001".into(),
        description: "Invoice identifier must be present and non-empty".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                let v = inv.id.value();
                if v.is_empty() {
                    Err("Invoice ID is empty — a non-empty invoice number is required".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── BT-002: Issue date must be present ───────────────────────────────
    // XSD-enforced (IssueDate is non-Option), but we still check for
    // completeness in the rule engine.
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT002".into(),
        description: "Invoice issue date must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // issue_date is a required struct field — always present.
                // This rule passes trivially but is included for completeness.
                let _date = &inv.issue_date;
                Ok(())
            })
        },
    });

    // ── BT-003: Invoice type code must be present ────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT003".into(),
        description: "Invoice type code must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.invoice_type_code {
                None => Err("Invoice type code is missing — required for Peppol BIS".into()),
                Some(tc) if tc.value().is_empty() => {
                    Err("Invoice type code is present but empty".into())
                }
                Some(_) => Ok(()),
            })
        },
    });

    // ── BT-005: Document currency code must be present ───────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT005".into(),
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

    // ── BT-007: Tax point date should be present (warning) ───────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT007".into(),
        description: "Tax point date should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.tax_point_date.is_none() {
                    Err("Tax point date is missing — should be provided when it differs from issue date".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── BT-009: Payment due date should be present (warning) ─────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT009".into(),
        description: "Payment due date should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.due_date.is_none() {
                    Err("Payment due date is missing — should be provided when payment terms apply".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── BT-022: Notes should provide context (warning) ───────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT022".into(),
        description: "Invoice notes should provide context when present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // Notes are optional — only warn if notes exist but are all empty,
                // or if there are no notes at all (informational).
                if inv.note.is_empty() {
                    Err("No invoice notes provided — consider adding context about the invoice".into())
                } else if inv.note.iter().all(|n| n.value().trim().is_empty()) {
                    Err("Invoice notes are present but all are empty — provide meaningful context".into())
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
    use chrono::NaiveDate;
    use peppol_common::rules::RuleEngine;
    use std::sync::Arc;
    use ubl_common::cbc;
    use ubl_documents::billing::Invoice;

    fn minimal_invoice() -> Invoice {
        use rust_decimal::Decimal;
        Invoice {
            id: cbc::ID::new("INV-001"),
            issue_date: cbc::IssueDate::new(NaiveDate::from_ymd_opt(2026, 6, 12).unwrap()),
            accounting_supplier_party: ubl_common::cac::SupplierParty {
                customer_assigned_account_id: None,
                additional_account_id: vec![],
                data_sending_capability: None,
                party: None,
                despatch_contact: None,
                accounting_contact: None,
                seller_contact: None,
            },
            legal_monetary_total: ubl_common::cac::LegalTotal {
                line_extension_amount: cbc::LineExtensionAmount::new(Decimal::ZERO, "ZAR"),
                tax_exclusive_amount: None,
                tax_inclusive_amount: None,
                allowance_total_amount: None,
                charge_total_amount: None,
                prepaid_amount: None,
                payable_rounding_amount: None,
                payable_amount: cbc::PayableAmount::new(Decimal::ZERO, "ZAR"),
            },
            ubl_version_id: None,
            customization_id: None,
            profile_id: None,
            profile_execution_id: None,
            copy_indicator: None,
            uuid: None,
            issue_time: None,
            due_date: None,
            tax_point_date: None,
            invoice_type_code: None,
            note: vec![],
            document_currency_code: None,
            tax_currency_code: None,
            pricing_currency_code: None,
            payment_currency_code: None,
            payment_alternative_currency_code: None,
            accounting_cost_code: None,
            accounting_cost: None,
            line_count_numeric: None,
            buyer_reference: None,
            default_language_code: None,
            invoice_period: vec![],
            order_reference: None,
            billing_reference: vec![],
            despatch_document_reference: vec![],
            delivery_note_document_reference: vec![],
            work_report_document_reference: vec![],
            receipt_document_reference: vec![],
            statement_document_reference: vec![],
            originator_document_reference: vec![],
            contract_document_reference: vec![],
            additional_document_reference: vec![],
            accounting_customer_party: None,
            payee_party: None,
            buyer_customer_party: None,
            seller_supplier_party: None,
            originator_customer_party: None,
            beneficiary_party: vec![],
            tax_representative_party: None,
            delivery: vec![],
            delivery_terms: None,
            payment_means: vec![],
            payment_terms: vec![],
            prepaid_payment: vec![],
            allowance_charge: vec![],
            tax_exchange_rate: None,
            pricing_exchange_rate: None,
            payment_exchange_rate: None,
            payment_alternative_exchange_rate: None,
            tax_total: vec![],
            invoice_line: vec![],
        }
    }

    #[test]
    fn test_bt001_id_empty_fails() {
        let mut inv = minimal_invoice();
        inv.id = cbc::ID::new("");
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT001"));
    }

    #[test]
    fn test_bt001_id_present_passes() {
        let inv = minimal_invoice();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(!failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT001"));
    }

    #[test]
    fn test_r001_profile_missing_warns() {
        let inv = minimal_invoice();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-R001"));
    }

    #[test]
    fn test_r001_profile_present_passes() {
        let mut inv = minimal_invoice();
        inv.profile_id = Some(cbc::ProfileID::new("urn:fdc:peppol.eu:2017:poacc:billing:01:1.0"));
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(!failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-R001"));
    }

    #[test]
    fn test_r004_customization_missing_fails() {
        let inv = minimal_invoice();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-R004"
            && f.severity == Some(Severity::Fatal)));
    }

    #[test]
    fn test_r004_customization_peppol_passes() {
        let mut inv = minimal_invoice();
        inv.customization_id = Some(cbc::CustomizationID::new(
            "urn:cen.eu:en16931:2017#compliant#urn:fdc:peppol.eu:2017:poacc:billing:3.0",
        ));
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(!failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-R004"));
    }

    #[test]
    fn test_r004_customization_wrong_fails() {
        let mut inv = minimal_invoice();
        inv.customization_id = Some(cbc::CustomizationID::new("urn:oasis:names:..."));
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-R004"));
    }

    #[test]
    fn test_bt003_type_code_missing_fails() {
        let inv = minimal_invoice();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT003"
            && f.severity == Some(Severity::Fatal)));
    }

    #[test]
    fn test_bt003_type_code_present_passes() {
        let mut inv = minimal_invoice();
        inv.invoice_type_code = Some(cbc::InvoiceTypeCode::new("380"));
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(!failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT003"));
    }

    #[test]
    fn test_bt005_currency_missing_fails() {
        let inv = minimal_invoice();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT005"
            && f.severity == Some(Severity::Fatal)));
    }

    #[test]
    fn test_bt005_currency_present_passes() {
        let mut inv = minimal_invoice();
        inv.document_currency_code = Some(cbc::DocumentCurrencyCode::new("EUR"));
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(!failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT005"));
    }

    #[test]
    fn test_bt007_tax_point_missing_warns() {
        let inv = minimal_invoice();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT007"
            && f.severity == Some(Severity::Warning)));
    }

    #[test]
    fn test_bt007_tax_point_present_passes() {
        let mut inv = minimal_invoice();
        inv.tax_point_date = Some(cbc::TaxPointDate::new(
            NaiveDate::from_ymd_opt(2026, 6, 12).unwrap(),
        ));
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(!failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT007"));
    }

    #[test]
    fn test_bt009_due_date_missing_warns() {
        let inv = minimal_invoice();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT009"
            && f.severity == Some(Severity::Warning)));
    }

    #[test]
    fn test_bt009_due_date_present_passes() {
        let mut inv = minimal_invoice();
        inv.due_date = Some(cbc::DueDate::new(
            NaiveDate::from_ymd_opt(2026, 7, 12).unwrap(),
        ));
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(!failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT009"));
    }

    #[test]
    fn test_bt022_no_notes_warns() {
        let inv = minimal_invoice();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT022"));
    }

    #[test]
    fn test_bt022_empty_notes_warns() {
        let mut inv = minimal_invoice();
        inv.note = vec![cbc::Note::new("  ")];
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT022"));
    }

    #[test]
    fn test_bt022_meaningful_notes_passes() {
        let mut inv = minimal_invoice();
        inv.note = vec![cbc::Note::new("Payment terms: Net 30 days")];
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(!failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT022"));
    }
}
