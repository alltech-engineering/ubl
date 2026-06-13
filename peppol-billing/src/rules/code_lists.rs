// Peppol BIS Billing 3.0 — Code List Validation Rules
//
// Validates that coded values belong to the correct UNCL / ISO code lists.
// Rules reference: EN16931 Schematron and Peppol BIS Billing 3.0 specification.

use peppol_common::codes::{
    currency_codes, invoice_type_codes, payment_means_codes, tax_category_codes, uncl2005_codes,
};
use peppol_common::participant::IcdCode;
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

    // ── CL006: InvoicePeriod DescriptionCode from UNCL2005 subset ─────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-CL006".into(),
        description: "InvoicePeriod DescriptionCode must be from UNCL2005 subset".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, period) in inv.invoice_period.iter().enumerate() {
                    for (j, dc) in period.description_code.iter().enumerate() {
                        if !uncl2005_codes().is_valid(&dc.value) {
                            return Err(format!(
                                "InvoicePeriod[{}].DescriptionCode[{}] '{}' is not a valid UNCL2005 code",
                                i + 1,
                                j + 1,
                                dc.value
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── CL007: InvoicePeriod DescriptionCode format validation ────────────
    // UNCL2005 codes must be numeric-only values.
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-CL007".into(),
        description: "InvoicePeriod DescriptionCode must be numeric".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, period) in inv.invoice_period.iter().enumerate() {
                    for (j, dc) in period.description_code.iter().enumerate() {
                        if !dc.value.chars().all(|c| c.is_ascii_digit()) {
                            return Err(format!(
                                "InvoicePeriod[{}].DescriptionCode[{}] '{}' is not numeric (UNCL2005 codes are numeric)",
                                i + 1,
                                j + 1,
                                dc.value
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── CL008: EndpointID schemeID must be from EAS code list ────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-CL008".into(),
        description: "EndpointID @schemeID must be a valid EAS code".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // Check supplier party endpoint_id
                if let Some(ref party) = inv.accounting_supplier_party.party {
                    if let Some(ref epid) = party.endpoint_id {
                        if let Some(ref scheme_id) = epid.0.scheme_id {
                            if IcdCode::by_eas(scheme_id).is_none() {
                                return Err(format!(
                                    "Supplier EndpointID schemeID '{}' is not a valid EAS code",
                                    scheme_id
                                ));
                            }
                        }
                    }
                }
                // Check customer party endpoint_id
                if let Some(ref customer) = inv.accounting_customer_party {
                    if let Some(ref party) = customer.party {
                        if let Some(ref epid) = party.endpoint_id {
                            if let Some(ref scheme_id) = epid.0.scheme_id {
                                if IcdCode::by_eas(scheme_id).is_none() {
                                    return Err(format!(
                                        "Customer EndpointID schemeID '{}' is not a valid EAS code",
                                        scheme_id
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
}
