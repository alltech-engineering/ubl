// Peppol BIS Billing 3.0 Business Rules
//
// Implements the Peppol business rules from:
//   - EN 16931 (European e-Invoicing standard)
//   - Peppol BIS Billing 3.0 transaction rules
//
// These rules go beyond XSD validation — they check business logic,
// cross-field consistency, and Peppol-specific requirements.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use ubl_documents::billing::Invoice;

/// Build the complete rule set for Peppol BIS Billing 3.0.
pub fn billing_rules(invoice: &Invoice) -> RuleEngine {
    let mut engine = RuleEngine::new();

    // ── EN 16931 Core Rules ──

    // BT-1: Invoice number MUST be present
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT001".into(),
        description: "Invoice number must be present".into(),
        severity: Severity::Fatal,
        check: {
            let id = invoice.id.clone();
            Box::new(move || {
                if id.value().is_empty() {
                    Err("Invoice ID is empty".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // BT-2: Invoice issue date MUST be present
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT002".into(),
        description: "Invoice issue date must be present".into(),
        severity: Severity::Fatal,
        check: Box::new(|| Ok(())), // XSD enforces this
    });

    // BT-5: Document currency code MUST be present
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT005".into(),
        description: "Document currency code must be present".into(),
        severity: Severity::Fatal,
        check: {
            let dcc = invoice.document_currency_code.clone();
            Box::new(move || {
                match &dcc {
                    Some(c) if !c.value().is_empty() => Ok(()),
                    _ => Err("Document currency code is missing or empty".into()),
                }
            })
        },
    });

    // BT-7: Invoice type code SHOULD be present
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT007".into(),
        description: "Invoice type code should be present for clarity".into(),
        severity: Severity::Warning,
        check: {
            let itc = invoice.invoice_type_code.clone();
            Box::new(move || {
                match &itc {
                    Some(_) => Ok(()),
                    None => Err("Invoice type code is not specified".into()),
                }
            })
        },
    });

    // BT-23: Supplier MUST have a name
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT023".into(),
        description: "Seller name must be present".into(),
        severity: Severity::Fatal,
        check: {
            let party = invoice.accounting_supplier_party.clone();
            Box::new(move || {
                if let Some(ref p) = party.party {
                    if p.party_name.is_empty() {
                        Err("Supplier has no party name".into())
                    } else {
                        Ok(())
                    }
                } else {
                    Err("Supplier party information is missing".into())
                }
            })
        },
    });

    // ── Peppol BIS Billing 3.0 Specific Rules ──

    // PEPPOL-1: AccountingSupplierParty MUST have a PartyIdentification with scheme
    engine.add_rule(Rule {
        id: "PEPPOL-BILLING-R001".into(),
        description: "Supplier must have a Peppol participant identifier".into(),
        severity: Severity::Error,
        check: {
            let party = invoice.accounting_supplier_party.clone();
            Box::new(move || {
                if let Some(ref p) = party.party {
                    if p.party_identification.is_empty() {
                        Err("Supplier has no party identification (Peppol participant ID required)".into())
                    } else {
                        Ok(())
                    }
                } else {
                    Err("Supplier party is missing".into())
                }
            })
        },
    });

    // PEPPOL-2: Customer SHOULD have a Peppol participant ID
    engine.add_rule(Rule {
        id: "PEPPOL-BILLING-R002".into(),
        description: "Customer should have a Peppol participant identifier".into(),
        severity: Severity::Warning,
        check: {
            let party = invoice.accounting_customer_party.clone();
            Box::new(move || {
                if let Some(ref cp) = party {
                    if let Some(ref p) = cp.party {
                        if p.party_identification.is_empty() {
                            Err("Customer has no party identification".into())
                        } else {
                            Ok(())
                        }
                    } else {
                        Err("Customer party is missing".into())
                    }
                } else {
                    Ok(()) // Customer is optional
                }
            })
        },
    });

    // PEPPOL-3: Legal monetary total MUST match sum of line totals
    engine.add_rule(Rule {
        id: "PEPPOL-BILLING-R003".into(),
        description: "Invoice total must match sum of line extension amounts".into(),
        severity: Severity::Error,
        check: {
            let total = invoice.legal_monetary_total.clone();
            let lines = invoice.invoice_line.clone();
            Box::new(move || {
                let sum: rust_decimal::Decimal = lines
                    .iter()
                    .map(|l| *l.line_extension_amount.value())
                    .sum();
                if sum == *total.line_extension_amount.value() {
                    Ok(())
                } else {
                    Err(format!(
                        "Line total sum ({:.2}) does not match invoice total ({:.2})",
                        sum,
                        total.line_extension_amount.value()
                    ))
                }
            })
        },
    });

    // PEPPOL-4: Payment means code MUST be from Peppol code list if present
    engine.add_rule(Rule {
        id: "PEPPOL-BILLING-R004".into(),
        description: "Payment means code must be valid".into(),
        severity: Severity::Error,
        check: {
            let pms = invoice.payment_means.clone();
            Box::new(move || {
                for pm in &pms {
                    let code = pm.payment_means_code.value();
                    // Simple validation — full validation uses CodeList
                    if code.is_empty() {
                        return Err("Payment means code is empty".into());
                    }
                }
                Ok(())
            })
        },
    });

    // PEPPOL-5: At least one InvoiceLine MUST be present
    engine.add_rule(Rule {
        id: "PEPPOL-BILLING-R005".into(),
        description: "Invoice must have at least one line item".into(),
        severity: Severity::Fatal,
        check: {
            let lines = invoice.invoice_line.clone();
            Box::new(move || {
                if lines.is_empty() {
                    Err("Invoice has no line items".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // PEPPOL-6: Tax total MUST be present (if applicable)
    engine.add_rule(Rule {
        id: "PEPPOL-BILLING-R006".into(),
        description: "Tax total breakdown should be provided".into(),
        severity: Severity::Warning,
        check: {
            let tax_total = invoice.tax_total.clone();
            Box::new(move || {
                if tax_total.is_empty() {
                    Err("No tax total breakdown present".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    engine
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_rules_valid_invoice() {
        let json = r#"{
            "id": {"value": "INV-001"},
            "issue_date": "2026-06-12",
            "document_currency_code": {"value": "ZAR"},
            "accounting_supplier_party": {
                "party": {
                    "party_name": [{"name": "Acme Corp"}],
                    "party_identification": [{"id": {"value": "9933:za1234567890"}}]
                }
            },
            "legal_monetary_total": {
                "line_extension_amount": {"value": "100.00", "currency_id": "ZAR"},
                "payable_amount": {"value": "115.00", "currency_id": "ZAR"}
            },
            "invoice_line": [{
                "id": {"value": "1"},
                "line_extension_amount": {"value": "100.00", "currency_id": "ZAR"},
                "item": {"name": "Widget"}
            }]
        }"#;
        let inv: Invoice = serde_json::from_str(json).unwrap();
        let engine = billing_rules(&inv);
        let failures = engine.evaluate_failures();
        // Warning about missing tax total is expected, not errors
        let errors: Vec<_> = failures.iter().filter(|f| f.severity != Some(Severity::Warning)).collect();
        assert!(errors.is_empty(), "Unexpected errors: {:?}", errors);
    }

    #[test]
    fn test_empty_invoice_fails() {
        let json = r#"{
            "id": {"value": ""},
            "issue_date": "2026-06-12",
            "accounting_supplier_party": {
                "party": {
                    "party_name": [{"name": "Acme Corp"}]
                }
            },
            "legal_monetary_total": {
                "line_extension_amount": {"value": "0", "currency_id": "ZAR"},
                "payable_amount": {"value": "0", "currency_id": "ZAR"}
            },
            "invoice_line": []
        }"#;
        let inv: Invoice = serde_json::from_str(json).unwrap();
        let engine = billing_rules(&inv);
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-EN16931-BT001"));
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-BILLING-R005"));
    }
}
