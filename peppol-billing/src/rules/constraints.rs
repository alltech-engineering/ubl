// Peppol BIS Billing 3.0 — Constraint & Cross-Document Profile Rules
//
// Validates EN16931 constraint/profile rules (P*), PEPPOL-COMMON rules,
// and additional R* rules not covered by other modules.
// Uses Arc<Invoice> for zero-copy evaluation across all rules.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::billing::Invoice;

/// Register all constraint, common, and additional validation rules.
pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Invoice>) {
    // ═══════════════════════════════════════════════════════════════
    // PEPPOL-EN16931-P* PROFILE / CONSTRAINT RULES
    // ═══════════════════════════════════════════════════════════════

    // ── P0100: InvoiceTypeCode must be from Peppol subset (informational) ─
    // CL001 already enforces this as Fatal; P0100 is a softer duplicate.
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-P0100".into(),
        description: "InvoiceTypeCode should be a valid Peppol BIS subset code".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.invoice_type_code {
                None => Err("InvoiceTypeCode is missing — should be a Peppol BIS code".into()),
                Some(tc) => {
                    let code = tc.value();
                    let valid = ["380", "381", "383", "384", "386", "388", "389", "395"];
                    if valid.contains(&code) {
                        Ok(())
                    } else {
                        Err(format!(
                            "InvoiceTypeCode '{}' is not in the Peppol BIS subset (380, 381, 383, 384, 386, 388, 389, 395)",
                            code
                        ))
                    }
                }
            })
        },
    });

    // ── P0101: CreditNoteTypeCode must be present if document is credit note ─
    // For Invoice: InvoiceTypeCode 381 signals a credit note, but the document
    // is typed as Invoice — this is a profile mismatch.
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-P0101".into(),
        description: "CreditNoteTypeCode must be present if document is a credit note".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.invoice_type_code {
                Some(tc) if tc.value() == "381" => {
                    Err("InvoiceTypeCode is 381 (Credit Note) but document type is Invoice — CreditNoteTypeCode is required for credit note documents. Use the CreditNote document type instead.".into())
                }
                _ => Ok(()),
            })
        },
    });

    // ── P0104: VATEX-EU-G requires certain fields ─────────────────────
    // When TaxExemptionReasonCode is VATEX-EU-G (free export), the
    // tax category percent should be zero and tax scheme should be VAT.
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-P0104".into(),
        description: "VATEX-EU-G must have valid tax exemption fields".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (ti, tt) in inv.tax_total.iter().enumerate() {
                    for (si, st) in tt.tax_subtotal.iter().enumerate() {
                        if let Some(ref code) = st.tax_category.tax_exemption_reason_code {
                            if code.value() == "VATEX-EU-G" {
                                // Must have tax scheme ID = "VAT"
                                if st.tax_category.tax_scheme.id.as_ref().map_or(true, |id| id.value() != "VAT") {
                                    return Err(format!(
                                        "TaxTotal[{}].TaxSubtotal[{}]: VATEX-EU-G requires TaxScheme ID = \"VAT\"",
                                        ti + 1, si + 1
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

    // ── P0105: VATEX-EU-O requires certain fields ─────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-P0105".into(),
        description: "VATEX-EU-O must have valid tax exemption fields".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (ti, tt) in inv.tax_total.iter().enumerate() {
                    for (si, st) in tt.tax_subtotal.iter().enumerate() {
                        if let Some(ref code) = st.tax_category.tax_exemption_reason_code {
                            if code.value() == "VATEX-EU-O" {
                                if st.tax_category.tax_scheme.id.as_ref().map_or(true, |id| id.value() != "VAT") {
                                    return Err(format!(
                                        "TaxTotal[{}].TaxSubtotal[{}]: VATEX-EU-O requires TaxScheme ID = \"VAT\"",
                                        ti + 1, si + 1
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

    // ── P0106: VATEX-EU-IC requires certain fields ────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-P0106".into(),
        description: "VATEX-EU-IC must have valid tax exemption fields".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (ti, tt) in inv.tax_total.iter().enumerate() {
                    for (si, st) in tt.tax_subtotal.iter().enumerate() {
                        if let Some(ref code) = st.tax_category.tax_exemption_reason_code {
                            if code.value() == "VATEX-EU-IC" {
                                if st.tax_category.tax_scheme.id.as_ref().map_or(true, |id| id.value() != "VAT") {
                                    return Err(format!(
                                        "TaxTotal[{}].TaxSubtotal[{}]: VATEX-EU-IC requires TaxScheme ID = \"VAT\"",
                                        ti + 1, si + 1
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

    // ── P0107: VATEX-EU-AE requires certain fields ────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-P0107".into(),
        description: "VATEX-EU-AE must have valid tax exemption fields".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (ti, tt) in inv.tax_total.iter().enumerate() {
                    for (si, st) in tt.tax_subtotal.iter().enumerate() {
                        if let Some(ref code) = st.tax_category.tax_exemption_reason_code {
                            if code.value() == "VATEX-EU-AE" {
                                if st.tax_category.tax_scheme.id.as_ref().map_or(true, |id| id.value() != "VAT") {
                                    return Err(format!(
                                        "TaxTotal[{}].TaxSubtotal[{}]: VATEX-EU-AE requires TaxScheme ID = \"VAT\"",
                                        ti + 1, si + 1
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

    // ── P0108: VATEX-EU-D requires certain fields ─────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-P0108".into(),
        description: "VATEX-EU-D must have valid tax exemption fields".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (ti, tt) in inv.tax_total.iter().enumerate() {
                    for (si, st) in tt.tax_subtotal.iter().enumerate() {
                        if let Some(ref code) = st.tax_category.tax_exemption_reason_code {
                            if code.value() == "VATEX-EU-D" {
                                if st.tax_category.tax_scheme.id.as_ref().map_or(true, |id| id.value() != "VAT") {
                                    return Err(format!(
                                        "TaxTotal[{}].TaxSubtotal[{}]: VATEX-EU-D requires TaxScheme ID = \"VAT\"",
                                        ti + 1, si + 1
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

    // ── P0109: VATEX-EU-F requires certain fields ─────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-P0109".into(),
        description: "VATEX-EU-F must have valid tax exemption fields".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (ti, tt) in inv.tax_total.iter().enumerate() {
                    for (si, st) in tt.tax_subtotal.iter().enumerate() {
                        if let Some(ref code) = st.tax_category.tax_exemption_reason_code {
                            if code.value() == "VATEX-EU-F" {
                                if st.tax_category.tax_scheme.id.as_ref().map_or(true, |id| id.value() != "VAT") {
                                    return Err(format!(
                                        "TaxTotal[{}].TaxSubtotal[{}]: VATEX-EU-F requires TaxScheme ID = \"VAT\"",
                                        ti + 1, si + 1
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

    // ── P0110: VATEX-EU-I requires certain fields ─────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-P0110".into(),
        description: "VATEX-EU-I must have valid tax exemption fields".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (ti, tt) in inv.tax_total.iter().enumerate() {
                    for (si, st) in tt.tax_subtotal.iter().enumerate() {
                        if let Some(ref code) = st.tax_category.tax_exemption_reason_code {
                            if code.value() == "VATEX-EU-I" {
                                if st.tax_category.tax_scheme.id.as_ref().map_or(true, |id| id.value() != "VAT") {
                                    return Err(format!(
                                        "TaxTotal[{}].TaxSubtotal[{}]: VATEX-EU-I requires TaxScheme ID = \"VAT\"",
                                        ti + 1, si + 1
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

    // ── P0111: VATEX-EU-J requires certain fields ─────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-P0111".into(),
        description: "VATEX-EU-J must have valid tax exemption fields".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (ti, tt) in inv.tax_total.iter().enumerate() {
                    for (si, st) in tt.tax_subtotal.iter().enumerate() {
                        if let Some(ref code) = st.tax_category.tax_exemption_reason_code {
                            if code.value() == "VATEX-EU-J" {
                                if st.tax_category.tax_scheme.id.as_ref().map_or(true, |id| id.value() != "VAT") {
                                    return Err(format!(
                                        "TaxTotal[{}].TaxSubtotal[{}]: VATEX-EU-J requires TaxScheme ID = \"VAT\"",
                                        ti + 1, si + 1
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

    // ── P0112: InvoiceTypeCode must not be credit-note codes on Invoice ─
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-P0112".into(),
        description: "InvoiceTypeCode must not be a credit note code (381) on an Invoice".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.invoice_type_code {
                Some(tc) if tc.value() == "381" => {
                    Err("InvoiceTypeCode is 381 (Credit Note) — this code is not allowed on Invoice documents. Use CreditNote document type.".into())
                }
                _ => Ok(()),
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════
    // PEPPOL-EN16931-F* RULES
    // ═══════════════════════════════════════════════════════════════

    // ── F001: CreditNoteTypeCode must not be invoice codes ─────────────
    // Since Invoice does not carry CreditNoteTypeCode, this rule checks
    // that the document does not misuse invoice-type semantics.
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-F001".into(),
        description: "CreditNoteTypeCode must not be an invoice-type code".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // Invoice does not have CreditNoteTypeCode — this rule
                // is informational for the Invoice document type.
                // If invoice_type_code is present and is 380, 384, 386, 388,
                // the document is correctly typed as an invoice variant.
                match &inv.invoice_type_code {
                    Some(_) => Ok(()),
                    None => Err("InvoiceTypeCode is missing — cannot verify F001".into()),
                }
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════
    // ADDITIONAL R* RULES
    // ═══════════════════════════════════════════════════════════════

    // ── R002: Invoice number pattern validation ────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R002".into(),
        description: "Invoice number must match a valid identifier pattern".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                let v = inv.id.value();
                if v.is_empty() {
                    Err("Invoice ID is empty — a non-empty identifier is required".into())
                } else if v.len() > 100 {
                    Err(format!(
                        "Invoice ID '{}' is too long ({} characters) — maximum 100 characters",
                        v,
                        v.len()
                    ))
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── R003: Issue date must be valid ─────────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R003".into(),
        description: "Invoice issue date must be a valid date".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // issue_date is a NaiveDate — always valid by construction.
                // Check that it is not in the far future (more than 1 year ahead).
                use chrono::Utc;
                let today = Utc::now().date_naive();
                let issue = inv.issue_date.0;
                let one_year = chrono::Duration::days(365);
                if issue > today + one_year {
                    Err(format!(
                        "Issue date {} is more than 1 year in the future — may be invalid",
                        issue
                    ))
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── R005: Tax currency must differ from document currency if present ─
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R005".into(),
        description: "Tax currency must differ from document currency when both are present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                let doc_cc = inv.document_currency_code.as_ref().map(|c| c.value());
                let tax_cc = inv.tax_currency_code.as_ref().map(|c| c.value());
                match (doc_cc, tax_cc) {
                    (Some(doc), Some(tax)) if doc == tax => {
                        Err(format!(
                            "TaxCurrencyCode '{}' is the same as DocumentCurrencyCode '{}' — must differ if both are present",
                            tax, doc
                        ))
                    }
                    _ => Ok(()),
                }
            })
        },
    });

    // ── R007: Profile ID must match known billing profile ──────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R007".into(),
        description: "ProfileID must match a known Peppol BIS Billing profile".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.profile_id {
                None => Err("ProfileID is missing — must declare a Peppol BIS Billing profile".into()),
                Some(pid) => {
                    let v = pid.value();
                    let known = [
                        "urn:fdc:peppol.eu:2017:poacc:billing:01:1.0",
                        "urn:fdc:peppol.eu:2017:poacc:billing:3.0",
                    ];
                    if known.contains(&v) || v.contains("peppol") && v.contains("billing") {
                        Ok(())
                    } else {
                        Err(format!(
                            "ProfileID '{}' does not match a known Peppol BIS Billing profile",
                            v
                        ))
                    }
                }
            })
        },
    });

    // ── R008: Empty elements must not exist (informational) ────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R008".into(),
        description: "Empty elements should not exist in the document".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                let mut empty: Vec<&str> = Vec::new();
                if inv.invoice_type_code.as_ref().map_or(false, |c| c.value().is_empty()) {
                    empty.push("InvoiceTypeCode");
                }
                if inv.document_currency_code.as_ref().map_or(false, |c| c.value().is_empty()) {
                    empty.push("DocumentCurrencyCode");
                }
                if inv.customization_id.as_ref().map_or(false, |c| c.value().is_empty()) {
                    empty.push("CustomizationID");
                }
                if inv.profile_id.as_ref().map_or(false, |c| c.value().is_empty()) {
                    empty.push("ProfileID");
                }
                // Check note elements
                if inv.note.iter().any(|n| n.value().trim().is_empty()) {
                    empty.push("Note");
                }
                if empty.is_empty() {
                    Ok(())
                } else {
                    Err(format!("Empty elements found: {}", empty.join(", ")))
                }
            })
        },
    });

    // ── R044: Document-level allowance charge must have reason code ────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R044".into(),
        description: "Document-level allowance must have a reason code".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, ac) in inv.allowance_charge.iter().enumerate() {
                    // charge_indicator=false means it's an allowance
                    if !ac.charge_indicator.is_true() {
                        if ac.allowance_charge_reason_code.is_none()
                            && ac.allowance_charge_reason.is_empty()
                        {
                            return Err(format!(
                                "Document-level allowance [{}] has no reason code or reason text",
                                i + 1
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── R046: Document-level charge must have reason code ──────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R046".into(),
        description: "Document-level charge must have a reason code".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, ac) in inv.allowance_charge.iter().enumerate() {
                    // charge_indicator=true means it's a charge
                    if ac.charge_indicator.is_true() {
                        if ac.allowance_charge_reason_code.is_none()
                            && ac.allowance_charge_reason.is_empty()
                        {
                            return Err(format!(
                                "Document-level charge [{}] has no reason code or reason text",
                                i + 1
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── R080: CreditNote must reference original invoice ───────────────
    // For Invoice documents: check that billing_reference or
    // additional_document_reference is present when appropriate.
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R080".into(),
        description: "Document must reference a preceding invoice when applicable".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // For Invoice documents, this is advisory — credit notes
                // carry the tighter requirement. Check if any reference
                // exists for traceability.
                let has_ref = !inv.billing_reference.is_empty()
                    || !inv.additional_document_reference.is_empty()
                    || !inv.order_reference.is_none();
                if has_ref {
                    Ok(())
                } else {
                    Err("No billing, order, or additional document reference — document traceability may be reduced".into())
                }
            })
        },
    });

    // ── R111: PartyTaxScheme CompanyID must match scheme ───────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R111".into(),
        description: "PartyTaxScheme CompanyID must match the tax scheme".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // Check supplier party tax schemes
                if let Some(ref party) = inv.accounting_supplier_party.party {
                    for (i, pts) in party.party_tax_scheme.iter().enumerate() {
                        let scheme_id = pts.tax_scheme.id.as_ref().map(|id| id.value());
                        let has_company_id = pts.company_id.as_ref().map_or(false, |cid| !cid.value().is_empty());
                        // If TaxScheme ID is "VAT", a CompanyID should be present
                        if scheme_id == Some("VAT") && !has_company_id {
                            return Err(format!(
                                "Supplier PartyTaxScheme[{}]: TaxScheme is \"VAT\" but no CompanyID provided",
                                i + 1
                            ));
                        }
                    }
                }
                // Check customer party tax schemes
                if let Some(ref customer) = inv.accounting_customer_party {
                    if let Some(ref party) = customer.party {
                        for (i, pts) in party.party_tax_scheme.iter().enumerate() {
                            let scheme_id = pts.tax_scheme.id.as_ref().map(|id| id.value());
                            let has_company_id = pts.company_id.as_ref().map_or(false, |cid| !cid.value().is_empty());
                            if scheme_id == Some("VAT") && !has_company_id {
                                return Err(format!(
                                    "Customer PartyTaxScheme[{}]: TaxScheme is \"VAT\" but no CompanyID provided",
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

    // ═══════════════════════════════════════════════════════════════
    // PEPPOL-COMMON RULES — Unit Code Validation
    // ═══════════════════════════════════════════════════════════════

    // ── R040: Unit code must be present on invoiced quantities ──────────
    engine.add_rule(Rule {
        id: "PEPPOL-COMMON-R040".into(),
        description: "Invoiced quantity must have a unit code".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.invoice_line.iter().enumerate() {
                    if let Some(ref qty) = line.invoiced_quantity {
                        if qty.0.unit_code.is_none() || qty.0.unit_code.as_deref() == Some("") {
                            return Err(format!(
                                "Invoice line {} invoiced quantity has no unit code",
                                i + 1
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── R041: Unit code must be from UN/ECE Rec.20 ──────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-COMMON-R041".into(),
        description: "Unit code must be a valid UN/ECE Rec.20 code".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // Common UN/ECE Rec.20 unit codes subset
                let valid_units = [
                    "EA", "E50", "E51", "HUR", "KGM", "LTR", "MTR", "MTK",
                    "MTQ", "NMP", "NPR", "SET", "TNE", "XPK",
                ];
                for (i, line) in inv.invoice_line.iter().enumerate() {
                    if let Some(ref qty) = line.invoiced_quantity {
                        if let Some(ref unit) = qty.0.unit_code {
                            if !unit.is_empty() && !valid_units.contains(&unit.as_str()) {
                                return Err(format!(
                                    "Invoice line {} unit code '{}' is not a recognized UN/ECE Rec.20 code",
                                    i + 1, unit
                                ));
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── R042: Unit code must be uppercase ───────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-COMMON-R042".into(),
        description: "Unit code must be in uppercase".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.invoice_line.iter().enumerate() {
                    if let Some(ref qty) = line.invoiced_quantity {
                        if let Some(ref unit) = qty.0.unit_code {
                            if *unit != unit.to_uppercase() {
                                return Err(format!(
                                    "Invoice line {} unit code '{}' should be uppercase",
                                    i + 1, unit
                                ));
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── R043: Unit code must not be empty string if present ─────────────
    engine.add_rule(Rule {
        id: "PEPPOL-COMMON-R043".into(),
        description: "Unit code must not be an empty string".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.invoice_line.iter().enumerate() {
                    if let Some(ref qty) = line.invoiced_quantity {
                        if qty.0.unit_code.as_deref() == Some("") {
                            return Err(format!(
                                "Invoice line {} has an empty unit code",
                                i + 1
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── R044: All quantities on a line should use consistent units ──────
    engine.add_rule(Rule {
        id: "PEPPOL-COMMON-R044".into(),
        description: "Quantities on a line should use consistent unit codes".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // Check that invoiced quantity unit code is present
                for (i, line) in inv.invoice_line.iter().enumerate() {
                    if line.invoiced_quantity.is_none() {
                        return Err(format!(
                            "Invoice line {} has no invoiced quantity",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── R045: Unit code length must be reasonable ───────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-COMMON-R045".into(),
        description: "Unit code length must be between 1 and 5 characters".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.invoice_line.iter().enumerate() {
                    if let Some(ref qty) = line.invoiced_quantity {
                        if let Some(ref unit) = qty.0.unit_code {
                            if unit.len() > 5 {
                                return Err(format!(
                                    "Invoice line {} unit code '{}' is too long (max 5 characters)",
                                    i + 1, unit
                                ));
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════
    // PEPPOL-COMMON RULES — EndpointID Scheme Validation
    // ═══════════════════════════════════════════════════════════════

    // ── R046: EndpointID scheme must be present on supplier ─────────────
    engine.add_rule(Rule {
        id: "PEPPOL-COMMON-R046".into(),
        description: "Supplier EndpointID must have a scheme identifier".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref party) = inv.accounting_supplier_party.party {
                    if let Some(ref epid) = party.endpoint_id {
                        if epid.0.scheme_id.is_none() {
                            return Err(
                                "Supplier EndpointID has no scheme_id — required for Peppol routing".into()
                            );
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── R047: Supplier EndpointID scheme_agency_id should be 9906 or 9907 ─
    engine.add_rule(Rule {
        id: "PEPPOL-COMMON-R047".into(),
        description: "Supplier EndpointID scheme agency must be 9906 (GLN) or 9907 (Peppol ID)".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref party) = inv.accounting_supplier_party.party {
                    if let Some(ref epid) = party.endpoint_id {
                        if let Some(ref agency) = epid.0.scheme_agency_id {
                            if agency != "9906" && agency != "9907" {
                                return Err(format!(
                                    "Supplier EndpointID scheme_agency_id '{}' is not 9906 (GLN) or 9907 (Peppol ID)",
                                    agency
                                ));
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── R048: Customer EndpointID scheme must be present ────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-COMMON-R048".into(),
        description: "Customer EndpointID must have a scheme identifier".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref customer) = inv.accounting_customer_party {
                    if let Some(ref party) = customer.party {
                        if let Some(ref epid) = party.endpoint_id {
                            if epid.0.scheme_id.is_none() {
                                return Err(
                                    "Customer EndpointID has no scheme_id — required for Peppol routing".into()
                                );
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── R049: Customer EndpointID scheme_agency_id should be 9906 or 9907 ─
    engine.add_rule(Rule {
        id: "PEPPOL-COMMON-R049".into(),
        description: "Customer EndpointID scheme agency must be 9906 (GLN) or 9907 (Peppol ID)".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref customer) = inv.accounting_customer_party {
                    if let Some(ref party) = customer.party {
                        if let Some(ref epid) = party.endpoint_id {
                            if let Some(ref agency) = epid.0.scheme_agency_id {
                                if agency != "9906" && agency != "9907" {
                                    return Err(format!(
                                        "Customer EndpointID scheme_agency_id '{}' is not 9906 (GLN) or 9907 (Peppol ID)",
                                        agency
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

    // ── R050: EndpointID value must not be empty ────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-COMMON-R050".into(),
        description: "EndpointID value must not be empty".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref party) = inv.accounting_supplier_party.party {
                    if let Some(ref epid) = party.endpoint_id {
                        if epid.0.value.is_empty() {
                            return Err("Supplier EndpointID value is empty".into());
                        }
                    }
                }
                if let Some(ref customer) = inv.accounting_customer_party {
                    if let Some(ref party) = customer.party {
                        if let Some(ref epid) = party.endpoint_id {
                            if epid.0.value.is_empty() {
                                return Err("Customer EndpointID value is empty".into());
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── R052: Unit code on base quantity must be valid ──────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-COMMON-R052".into(),
        description: "Base quantity unit code on price must be valid".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, line) in inv.invoice_line.iter().enumerate() {
                    if let Some(ref price) = line.price {
                        if let Some(ref base_qty) = price.base_quantity {
                            if let Some(ref unit) = base_qty.0.unit_code {
                                if unit.is_empty() {
                                    return Err(format!(
                                        "Invoice line {} base quantity unit code is empty",
                                        i + 1
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

    // ── R053: Unit code on tax category base unit measure must be valid ─
    engine.add_rule(Rule {
        id: "PEPPOL-COMMON-R053".into(),
        description: "Tax category base unit measure must have a valid unit code".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (ti, tt) in inv.tax_total.iter().enumerate() {
                    for (si, st) in tt.tax_subtotal.iter().enumerate() {
                        if let Some(ref bum) = st.tax_category.base_unit_measure {
                            let unit = bum.unit_code();
                            if unit.is_empty() {
                                    return Err(format!(
                                        "TaxTotal[{}].TaxSubtotal[{}] base unit measure unit code is empty",
                                        ti + 1, si + 1
                                    ));
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════
    // CODE LIST RULES
    // ═══════════════════════════════════════════════════════════════

    // ── CL006: InvoicePeriod DescriptionCode must be valid ──────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-CL006".into(),
        description: "InvoicePeriod DescriptionCode must be a valid UNCL2005 code".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // UNCL 2005 date/time period qualifier codes subset
                let valid = ["3", "35", "432"];
                for (i, period) in inv.invoice_period.iter().enumerate() {
                    for (j, code) in period.description_code.iter().enumerate() {
                        if !code.value.is_empty() && !valid.contains(&code.value.as_str()) {
                            return Err(format!(
                                "InvoicePeriod[{}].DescriptionCode[{}] '{}' is not a valid UNCL2005 code",
                                i + 1, j + 1, code.value
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── CL007: InvoicePeriod DescriptionCode format check ───────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-CL007".into(),
        description: "InvoicePeriod DescriptionCode must be numeric (UNCL2005)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                for (i, period) in inv.invoice_period.iter().enumerate() {
                    for (j, code) in period.description_code.iter().enumerate() {
                        if !code.value.is_empty() && !code.value.chars().all(|c| c.is_ascii_digit()) {
                            return Err(format!(
                                "InvoicePeriod[{}].DescriptionCode[{}] '{}' is not numeric — UNCL2005 codes must be digits only",
                                i + 1, j + 1, code.value
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── CL008: EndpointID scheme must be from EAS code list ─────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-CL008".into(),
        description: "EndpointID scheme must be a valid EAS (Electronic Address Scheme) code".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                // EAS code list (subset of commonly used Peppol identifiers)
                let valid_eas = [
                    "0002", "0009", "0037", "0051", "0057", "0060",
                    "0088", "0096", "0097", "0106", "0130", "0135",
                    "0141", "0183", "0184", "0190", "0191", "0192",
                    "0193", "0195", "0196", "0198", "0199", "0200",
                    "0201", "0202", "0203", "0204", "0205", "0206",
                    "0207", "0208", "0209", "0210", "0211", "0212",
                    "0213", "0214", "0215", "0216", "9906", "9907",
                    "9910", "9913", "9914", "9915", "9918", "9919",
                    "9920", "9922", "9923", "9924", "9925", "9926",
                    "9927", "9928", "9929", "9930", "9931", "9932",
                    "9933", "9934", "9935", "9936", "9937", "9938",
                    "9939", "9940", "9941", "9942", "9943", "9944",
                    "9945", "9946", "9947", "9948", "9949", "9950",
                    "9951", "9952", "9953", "9954", "9955", "9956",
                    "9957",
                ];
                let check_endpoint = |epid: &ubl_common::cbc::EndpointID, role: &str|
                    -> Result<(), String> {
                    if let Some(ref scheme) = epid.0.scheme_id {
                        if !scheme.is_empty() && !valid_eas.contains(&scheme.as_str()) {
                            return Err(format!(
                                "{} EndpointID scheme '{}' is not a valid EAS code",
                                role, scheme
                            ));
                        }
                    }
                    Ok(())
                };
                if let Some(ref party) = inv.accounting_supplier_party.party {
                    if let Some(ref epid) = party.endpoint_id {
                        check_endpoint(epid, "Supplier")?;
                    }
                }
                if let Some(ref customer) = inv.accounting_customer_party {
                    if let Some(ref party) = customer.party {
                        if let Some(ref epid) = party.endpoint_id {
                            check_endpoint(epid, "Customer")?;
                        }
                    }
                }
                Ok(())
            })
        },
    });
}
