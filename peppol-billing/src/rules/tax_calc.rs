// Peppol BIS Billing 3.0 — Tax Calculation Validation Rules
//
// Validates tax totals, subtotals, and line/total consistency.
// Rules reference: EN16931 Schematron and Peppol BIS Billing 3.0 specification.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::billing::Invoice;

/// Register all tax calculation validation rules.
pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Invoice>) {
    // ── BT110: Tax total breakdown must be present ───────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT110".into(),
        description: "Invoice tax total breakdown must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.tax_total.is_empty() {
                    Err("Tax total breakdown is missing — at least one TaxTotal is required".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── R053: Each tax subtotal must have a tax category ID ─────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R053".into(),
        description: "Each TaxSubtotal must have a tax category identifier".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (ti, tt) in inv.tax_total.iter().enumerate() {
                    for (si, st) in tt.tax_subtotal.iter().enumerate() {
                        if st.tax_category.id.is_none() {
                            return Err(format!(
                                "TaxTotal[{}].TaxSubtotal[{}] is missing a tax category ID",
                                ti + 1,
                                si + 1
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── R054: Tax amount must equal sum of subtotal amounts ─────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R054".into(),
        description: "TaxTotal TaxAmount must equal the sum of TaxSubtotal amounts".into(),
        severity: Severity::Error,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                use rust_decimal::Decimal;
                for (i, tt) in inv.tax_total.iter().enumerate() {
                    let declared: Decimal = *tt.tax_amount.value();
                    let sum: Decimal = tt
                        .tax_subtotal
                        .iter()
                        .map(|st| *st.tax_amount.value())
                        .sum();
                    if declared != sum {
                        return Err(format!(
                            "TaxTotal[{}]: declared tax amount {} does not match sum of subtotals {}",
                            i + 1,
                            declared,
                            sum
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── R055: Line total sum must match declared total ──────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R055".into(),
        description: "Sum of invoice line extension amounts must match LegalMonetaryTotal LineExtensionAmount".into(),
        severity: Severity::Error,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                use rust_decimal::Decimal;
                let line_sum: Decimal = inv
                    .invoice_line
                    .iter()
                    .map(|line| *line.line_extension_amount.value())
                    .sum();
                let declared: Decimal = *inv.legal_monetary_total.line_extension_amount.value();
                if line_sum != declared {
                    Err(format!(
                        "Sum of line extension amounts {} does not match declared total {}",
                        line_sum,
                        declared
                    ))
                } else {
                    Ok(())
                }
            })
        },
    });
}
