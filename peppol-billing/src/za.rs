// Peppol BIS Billing 3.0 — South African National Rules
//
// South Africa-specific Peppol requirements beyond the EU base rules.
// These reflect SARS (South African Revenue Service) e-invoicing mandates.
//
// Key ZA requirements:
//   - SARS VAT number (9950) on both supplier and customer
//   - Tax invoice wording ("TAX INVOICE" for amounts > R50)
//   - ZAR currency for domestic transactions
//   - Original vs copy indicator
//   - CIPC registration number (9933) as party identification

use peppol_common::rules::{Rule, Severity};
use ubl_documents::billing::Invoice;

/// Build the ZA-specific national rules for Peppol Billing 3.0.
pub fn national_rules(invoice: &Invoice) -> Vec<Rule> {
    let mut rules = Vec::new();

    // ZA-R001: Document currency MUST be ZAR for domestic transactions
    rules.push(Rule {
        id: "PEPPOL-ZA-R001".into(),
        description: "South African domestic invoices must use ZAR currency".into(),
        severity: Severity::Warning,
        check: {
            let dcc = invoice.document_currency_code.clone();
            Box::new(move || {
                match &dcc {
                    Some(c) if c.value() == "ZAR" => Ok(()),
                    Some(c) => Err(format!("Currency is {} — ZAR expected for domestic transactions", c.value())),
                    None => Err("No document currency specified".into()),
                }
            })
        },
    });

    // ZA-R002: Supplier SHOULD have a SARS VAT number (ICD 9950)
    rules.push(Rule {
        id: "PEPPOL-ZA-R002".into(),
        description: "Supplier should provide SARS VAT number (ICD 9950)".into(),
        severity: Severity::Warning,
        check: {
            let party = invoice.accounting_supplier_party.clone();
            Box::new(move || {
                if let Some(ref p) = party.party {
                    let has_vat = p.party_tax_scheme.iter().any(|pts| {
                        pts.company_id.as_ref().map_or(false, |cid| !cid.value().is_empty())
                    });
                    if has_vat {
                        Ok(())
                    } else {
                        Err("Supplier VAT number not provided — required by SARS".into())
                    }
                } else {
                    Err("Supplier party missing".into())
                }
            })
        },
    });

    // ZA-R003: If InvoiceTypeCode is present, it should be "388" (Tax Invoice) for SA
    rules.push(Rule {
        id: "PEPPOL-ZA-R003".into(),
        description: "SA invoices should carry InvoiceTypeCode 388 (Tax Invoice)".into(),
        severity: Severity::Warning,
        check: {
            let itc = invoice.invoice_type_code.clone();
            Box::new(move || {
                match &itc {
                    Some(c) if c.value() == "388" => Ok(()),
                    Some(c) => Err(format!("InvoiceTypeCode is {} — 388 (Tax Invoice) recommended for SA", c.value())),
                    None => Err("No InvoiceTypeCode specified".into()),
                }
            })
        },
    });

    // ZA-R004: Tax subtotals should use VAT category code "S" (Standard Rate) for SA
    rules.push(Rule {
        id: "PEPPOL-ZA-R004".into(),
        description: "SA VAT should use category code S, Z, or E".into(),
        severity: Severity::Warning,
        check: {
            let tax_totals = invoice.tax_total.clone();
            Box::new(move || {
                for tt in &tax_totals {
                    for st in &tt.tax_subtotal {
                        let code = st.tax_category.id.as_ref().map(|id| id.value().to_string()).unwrap_or_default();
                        if !["S", "Z", "E"].contains(&code.as_str()) {
                            return Err(format!(
                                "Unexpected VAT category code '{}' — SA typically uses S, Z, or E",
                                code
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_za_currency_rule() {
        let json = r#"{
            "id": {"value": "INV-001"},
            "issue_date": "2026-06-12",
            "document_currency_code": {"value": "USD"},
            "accounting_supplier_party": {
                "party": {
                    "party_name": [{"name": "Acme Corp"}]
                }
            },
            "legal_monetary_total": {
                "line_extension_amount": {"value": "100.00", "currency_id": "USD"},
                "payable_amount": {"value": "100.00", "currency_id": "USD"}
            },
            "invoice_line": [{
                "id": {"value": "1"},
                "line_extension_amount": {"value": "100.00", "currency_id": "USD"},
                "item": {"name": "Widget"}
            }]
        }"#;
        let inv: Invoice = serde_json::from_str(json).unwrap();
        let rules = national_rules(&inv);
        let outcomes: Vec<_> = rules.iter().map(|r| r.evaluate()).collect();
        let za1 = outcomes.iter().find(|o| o.rule_id == "PEPPOL-ZA-R001").unwrap();
        assert!(!za1.is_ok(), "ZA-R001 should warn about USD currency");
        assert!(za1.message.contains("USD"));
    }
}
