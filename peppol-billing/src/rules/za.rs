// Peppol BIS Billing 3.0 — South African National Rules
//
// SARS (South African Revenue Service) e-invoicing mandates.
// Applied on top of EN16931 + Peppol BIS Billing 3.0 base rules.
//
// Key ZA requirements:
//   - ZAR currency for domestic transactions
//   - SARS VAT number (9950) on supplier
//   - Tax Invoice type code (388)
//   - Standard VAT categories (S, Z, E)

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::billing::Invoice;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Invoice>) {
    // ZA-R001: Document currency MUST be ZAR for domestic transactions
    engine.add_rule(Rule {
        id: "PEPPOL-ZA-R001".into(),
        description: "South African domestic invoices must use ZAR currency".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.document_currency_code {
                Some(c) if c.value() == "ZAR" => Ok(()),
                Some(c) => Err(format!(
                    "Currency is {} — ZAR expected for domestic transactions",
                    c.value()
                )),
                None => Err("No document currency specified".into()),
            })
        },
    });

    // ZA-R002: Supplier SHOULD have a SARS VAT number
    engine.add_rule(Rule {
        id: "PEPPOL-ZA-R002".into(),
        description: "Supplier should provide SARS VAT number (ICD 9950)".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref p) = inv.accounting_supplier_party.party {
                    let has_vat = p.party_tax_scheme.iter().any(|pts| {
                        pts.company_id
                            .as_ref()
                            .map_or(false, |cid| !cid.value().is_empty())
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

    // ZA-R003: InvoiceTypeCode should be "388" (Tax Invoice) for SA
    engine.add_rule(Rule {
        id: "PEPPOL-ZA-R003".into(),
        description: "SA invoices should carry InvoiceTypeCode 388 (Tax Invoice)".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.invoice_type_code {
                Some(c) if c.value() == "388" => Ok(()),
                Some(c) => Err(format!(
                    "InvoiceTypeCode is {} — 388 (Tax Invoice) recommended for SA",
                    c.value()
                )),
                None => Err("No InvoiceTypeCode specified".into()),
            })
        },
    });

    // ZA-R004: Tax subtotals should use SA VAT categories
    engine.add_rule(Rule {
        id: "PEPPOL-ZA-R004".into(),
        description: "SA VAT should use category code S, Z, or E".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for tt in &inv.tax_total {
                    for st in &tt.tax_subtotal {
                        let code = st
                            .tax_category
                            .id
                            .as_ref()
                            .map(|id| id.value().to_string())
                            .unwrap_or_default();
                        if !["S", "Z", "E"].contains(&code.as_str()) {
                            return Err(format!(
                                "Unexpected VAT category '{}' — SA typically uses S, Z, or E",
                                code
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });
}
