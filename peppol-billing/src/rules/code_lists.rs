// Peppol BIS Billing 3.0 — Code List Validation Rules
//
// Validates that coded values belong to the correct UNCL / ISO code lists.
// Rules reference: EN16931 Schematron and Peppol BIS Billing 3.0 specification.

use peppol_common::codes::{
    currency_codes, invoice_type_codes, payment_means_codes, tax_category_codes,
};
use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::billing::Invoice;

/// Register all code list validation rules.
pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Invoice>) {
    // ── CL001: InvoiceTypeCode must be in UNCL1001 ───────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-CL001".into(),
        description: "InvoiceTypeCode must be a valid UNCL1001 code".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.invoice_type_code {
                None => Err("InvoiceTypeCode is missing — required for Peppol BIS".into()),
                Some(tc) => {
                    let code = tc.value();
                    if invoice_type_codes().is_valid(code) {
                        Ok(())
                    } else {
                        Err(format!(
                            "InvoiceTypeCode '{}' is not a valid UNCL1001 code",
                            code
                        ))
                    }
                }
            })
        },
    });

    // ── CL002: PaymentMeansCode must be in UNCL4461 ──────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-CL002".into(),
        description: "PaymentMeansCode must be a valid UNCL4461 code".into(),
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

    // ── CL003: TaxCategory must be in UNCL5305 ───────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-CL003".into(),
        description: "TaxCategory must be a valid UNCL5305 code".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (ti, tt) in inv.tax_total.iter().enumerate() {
                    for (si, st) in tt.tax_subtotal.iter().enumerate() {
                        match &st.tax_category.id {
                            None => {
                                return Err(format!(
                                    "TaxTotal[{}].TaxSubtotal[{}] TaxCategory ID is missing",
                                    ti + 1,
                                    si + 1
                                ));
                            }
                            Some(id) => {
                                let code = id.value();
                                if !tax_category_codes().is_valid(code) {
                                    return Err(format!(
                                        "TaxTotal[{}].TaxSubtotal[{}] TaxCategory '{}' is not a valid UNCL5305 code",
                                        ti + 1,
                                        si + 1,
                                        code
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

    // ── CL004: DocumentCurrencyCode must be valid ISO4217 ────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-CL004".into(),
        description: "DocumentCurrencyCode must be a valid ISO4217 currency code".into(),
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
                            "DocumentCurrencyCode '{}' is not a valid ISO4217 currency code",
                            code
                        ))
                    }
                }
            })
        },
    });
}
