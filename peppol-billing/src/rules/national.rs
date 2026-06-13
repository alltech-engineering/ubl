// Peppol BIS Billing 3.0 — National Business Rules
//
// Country-specific rules that only apply when the supplier or customer
// is in a specific jurisdiction. Skips each rule silently when the
// supplier country doesn't match the rule's target country.
//
// Rules reference: National Peppol BIS Billing 3.0 Schematron extensions.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::billing::Invoice;

/// Helper: returns true if the supplier's country code matches the given ISO code.
fn supplier_country_is(inv: &Invoice, country_code: &str) -> bool {
    inv.accounting_supplier_party
        .party
        .as_ref()
        .and_then(|p| p.postal_address.as_ref())
        .and_then(|a| a.country.as_ref())
        .and_then(|c| c.identification_code.as_ref())
        .map_or(false, |code| code.value() == country_code)
}

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Invoice>) {
    // ═══════════════════════════════════════════════════════════════════════
    // DENMARK (DK) — DK-R-002 through DK-R-017
    // ═══════════════════════════════════════════════════════════════════════

    // ── DK-R-002: DK supplier must have CVR number ─────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-002".into(),
        description: "DK supplier must have CVR number in PartyLegalEntity CompanyID".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => {
                        Err("DK supplier party is missing — CVR number cannot be verified".into())
                    }
                    Some(party) => {
                        let has_cvr = party.party_legal_entity.iter().any(|ple| {
                            ple.company_id
                                .as_ref()
                                .map_or(false, |cid| !cid.value().is_empty())
                        });
                        if has_cvr {
                            Ok(())
                        } else {
                            Err("DK supplier must have a CVR number in PartyLegalEntity".into())
                        }
                    }
                }
            })
        },
    });

    // ── DK-R-003: DK supplier CVR format check ────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-003".into(),
        description: "DK CVR number must be exactly 8 digits".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Ok(()),
                    Some(party) => {
                        for ple in &party.party_legal_entity {
                            if let Some(cid) = &ple.company_id {
                                let v = cid.value();
                                if !v.is_empty() {
                                    if v.len() != 8 || !v.chars().all(|c| c.is_ascii_digit()) {
                                        return Err(format!(
                                            "DK CVR number '{}' is invalid — must be exactly 8 digits",
                                            v
                                        ));
                                    }
                                }
                            }
                        }
                        Ok(())
                    }
                }
            })
        },
    });

    // ── DK-R-004: AllowanceCharge reason code required for DK ─────────
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-004".into(),
        description: "DK allowance/charge must have reason code".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                for (i, ac) in inv.allowance_charge.iter().enumerate() {
                    if ac.allowance_charge_reason_code.is_none() {
                        return Err(format!(
                            "AllowanceCharge[{}] has no reason code — required for DK",
                            i + 1
                        ));
                    }
                }
                Ok(())
            })
        },
    });

    // ── DK-R-005: DK supplier PartyLegalEntity required ────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-005".into(),
        description: "DK supplier must have PartyLegalEntity".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("DK supplier party is missing".into()),
                    Some(party) => {
                        if party.party_legal_entity.is_empty() {
                            Err("DK supplier must have PartyLegalEntity".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ── DK-R-006: DK customer PartyIdentification required if present ──
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-006".into(),
        description: "DK customer must have PartyIdentification when present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                match &inv.accounting_customer_party {
                    None => Ok(()),
                    Some(customer) => match &customer.party {
                        None => Ok(()),
                        Some(party) => {
                            if party.party_identification.is_empty() {
                                Err("DK customer party present but missing PartyIdentification"
                                    .into())
                            } else {
                                Ok(())
                            }
                        }
                    },
                }
            })
        },
    });

    // ── DK-R-007: DK supplier must have VAT in PartyTaxScheme ──────────
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-007".into(),
        description: "DK supplier must have VAT number in PartyTaxScheme".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("DK supplier party is missing".into()),
                    Some(party) => {
                        let has_vat = party.party_tax_scheme.iter().any(|pts| {
                            pts.company_id
                                .as_ref()
                                .map_or(false, |cid| !cid.value().is_empty())
                        });
                        if has_vat {
                            Ok(())
                        } else {
                            Err("DK supplier must have VAT number in PartyTaxScheme".into())
                        }
                    }
                }
            })
        },
    });

    // ── DK-R-008: DK supplier postal address required ──────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-008".into(),
        description: "DK supplier must have postal address".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("DK supplier party is missing".into()),
                    Some(party) => {
                        if party.postal_address.is_none() {
                            Err("DK supplier must have a postal address".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ── DK-R-009: DK supplier street name required ────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-009".into(),
        description: "DK supplier must have street name".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("DK supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("DK supplier postal address is missing".into()),
                        Some(addr) => {
                            if addr.street_name.is_none() {
                                Err("DK supplier must have street name".into())
                            } else {
                                Ok(())
                            }
                        }
                    },
                }
            })
        },
    });

    // ── DK-R-010: DK supplier city name required ──────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-010".into(),
        description: "DK supplier must have city name".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("DK supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("DK supplier postal address is missing".into()),
                        Some(addr) => {
                            if addr.city_name.is_none() {
                                Err("DK supplier must have city name".into())
                            } else {
                                Ok(())
                            }
                        }
                    },
                }
            })
        },
    });

    // ── DK-R-011: DK supplier postal zone required ────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-011".into(),
        description: "DK supplier must have postal zone".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("DK supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("DK supplier postal address is missing".into()),
                        Some(addr) => {
                            if addr.postal_zone.is_none() {
                                Err("DK supplier must have postal zone".into())
                            } else {
                                Ok(())
                            }
                        }
                    },
                }
            })
        },
    });

    // ── DK-R-013: DK supplier country code must be "DK" ────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-013".into(),
        description: "DK supplier country code must be DK".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("DK supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("DK supplier postal address is missing".into()),
                        Some(addr) => match &addr.country {
                            None => Err("DK supplier country is missing".into()),
                            Some(country) => match &country.identification_code {
                                None => Err("DK supplier country code is missing".into()),
                                Some(code) => {
                                    if code.value() != "DK" {
                                        Err(format!(
                                            "DK supplier country code is '{}' — must be DK",
                                            code.value()
                                        ))
                                    } else {
                                        Ok(())
                                    }
                                }
                            },
                        },
                    },
                }
            })
        },
    });

    // ── DK-R-014: DK customer country code must be "DK" if present ────
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-014".into(),
        description: "DK customer country code must be DK when present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                match &inv.accounting_customer_party {
                    None => Ok(()),
                    Some(customer) => match &customer.party {
                        None => Ok(()),
                        Some(party) => match &party.postal_address {
                            None => Ok(()),
                            Some(addr) => match &addr.country {
                                None => Ok(()),
                                Some(country) => match &country.identification_code {
                                    None => Ok(()),
                                    Some(code) => {
                                        if code.value() != "DK" {
                                            Err(format!(
                                                "DK customer country code is '{}' — expected DK",
                                                code.value()
                                            ))
                                        } else {
                                            Ok(())
                                        }
                                    }
                                },
                            },
                        },
                    },
                }
            })
        },
    });

    // ── DK-R-016: DK supplier postal zone format check ────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-016".into(),
        description: "DK postal zone must be 4 digits".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Ok(()),
                    Some(party) => match &party.postal_address {
                        None => Ok(()),
                        Some(addr) => match &addr.postal_zone {
                            None => Ok(()),
                            Some(zone) => {
                                let v = zone.value();
                                if v.len() != 4 || !v.chars().all(|c| c.is_ascii_digit()) {
                                    Err(format!(
                                        "DK postal zone '{}' is invalid — must be 4 digits",
                                        v
                                    ))
                                } else {
                                    Ok(())
                                }
                            }
                        },
                    },
                }
            })
        },
    });

    // ── DK-R-017: DK supplier country subentity required ──────────────
    engine.add_rule(Rule {
        id: "PEPPOL-DK-R-017".into(),
        description: "DK supplier must have country subentity".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "DK") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("DK supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("DK supplier postal address is missing".into()),
                        Some(addr) => {
                            if addr.country_subentity.is_none() {
                                Err("DK supplier must have country subentity".into())
                            } else {
                                Ok(())
                            }
                        }
                    },
                }
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════════════
    // ITALY (IT) — IT-R-001 through IT-R-004
    // ═══════════════════════════════════════════════════════════════════════

    // ── IT-R-001: IT supplier VAT required ────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IT-R-001".into(),
        description: "IT supplier must have VAT number (Partita IVA)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IT") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("IT supplier party is missing".into()),
                    Some(party) => {
                        let has_vat = party.party_tax_scheme.iter().any(|pts| {
                            pts.company_id
                                .as_ref()
                                .map_or(false, |cid| !cid.value().is_empty())
                        });
                        if has_vat {
                            Ok(())
                        } else {
                            Err(
                                "IT supplier must have VAT number (Partita IVA) in PartyTaxScheme"
                                    .into(),
                            )
                        }
                    }
                }
            })
        },
    });

    // ── IT-R-002: IT VAT format check ─────────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IT-R-002".into(),
        description: "IT VAT number must be 11 digits".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IT") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Ok(()),
                    Some(party) => {
                        for pts in &party.party_tax_scheme {
                            if let Some(cid) = &pts.company_id {
                                let v = cid.value();
                                if !v.is_empty() {
                                    if v.len() != 11 || !v.chars().all(|c| c.is_ascii_digit()) {
                                        return Err(format!(
                                            "IT VAT number '{}' is invalid — must be 11 digits",
                                            v
                                        ));
                                    }
                                }
                            }
                        }
                        Ok(())
                    }
                }
            })
        },
    });

    // ── IT-R-003: IT supplier fiscal code required ────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IT-R-003".into(),
        description: "IT supplier must have PartyIdentification (Codice Fiscale)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IT") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("IT supplier party is missing".into()),
                    Some(party) => {
                        if party.party_identification.is_empty() {
                            Err("IT supplier must have PartyIdentification (Codice Fiscale)".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ── IT-R-004: IT invoice type code must be valid ──────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IT-R-004".into(),
        description: "IT invoice must have valid InvoiceTypeCode (TD01-TD28)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IT") {
                    return Ok(());
                }
                match &inv.invoice_type_code {
                    None => Err("IT invoice must have InvoiceTypeCode".into()),
                    Some(tc) => {
                        let code = tc.value();
                        // Italian FatturaPA type codes: TD01-TD28
                        let valid = matches!(
                            code,
                            "TD01"
                                | "TD02"
                                | "TD03"
                                | "TD04"
                                | "TD05"
                                | "TD06"
                                | "TD07"
                                | "TD08"
                                | "TD09"
                                | "TD10"
                                | "TD11"
                                | "TD12"
                                | "TD13"
                                | "TD14"
                                | "TD15"
                                | "TD16"
                                | "TD17"
                                | "TD18"
                                | "TD19"
                                | "TD20"
                                | "TD21"
                                | "TD22"
                                | "TD23"
                                | "TD24"
                                | "TD25"
                                | "TD26"
                                | "TD27"
                                | "TD28"
                        );
                        if valid {
                            Ok(())
                        } else {
                            Err(format!(
                                "IT InvoiceTypeCode '{}' is not a valid FatturaPA type (TD01-TD28)",
                                code
                            ))
                        }
                    }
                }
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════════════
    // NORWAY (NO) — NO-R-001, NO-R-002
    // ═══════════════════════════════════════════════════════════════════════

    // ── NO-R-001: NO supplier OrgNr required ──────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-NO-R-001".into(),
        description: "NO supplier must have organisation number".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "NO") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("NO supplier party is missing".into()),
                    Some(party) => {
                        let has_orgnr = party.party_legal_entity.iter().any(|ple| {
                            ple.company_id
                                .as_ref()
                                .map_or(false, |cid| !cid.value().is_empty())
                        });
                        if has_orgnr {
                            Ok(())
                        } else {
                            Err(
                                "NO supplier must have organisation number in PartyLegalEntity"
                                    .into(),
                            )
                        }
                    }
                }
            })
        },
    });

    // ── NO-R-002: NO OrgNr format check ───────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-NO-R-002".into(),
        description: "NO organisation number must be 9 digits".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "NO") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Ok(()),
                    Some(party) => {
                        for ple in &party.party_legal_entity {
                            if let Some(cid) = &ple.company_id {
                                let v = cid.value();
                                if !v.is_empty() {
                                    if v.len() != 9 || !v.chars().all(|c| c.is_ascii_digit()) {
                                        return Err(format!(
                                            "NO organisation number '{}' is invalid — must be 9 digits",
                                            v
                                        ));
                                    }
                                }
                            }
                        }
                        Ok(())
                    }
                }
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════════════
    // SWEDEN (SE) — SE-R-001 through SE-R-013
    // ═══════════════════════════════════════════════════════════════════════

    // ── SE-R-001: SE supplier VAT required ────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-SE-R-001".into(),
        description: "SE supplier must have VAT number (Momsregistreringsnummer)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "SE") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("SE supplier party is missing".into()),
                    Some(party) => {
                        let has_vat = party.party_tax_scheme.iter().any(|pts| {
                            pts.company_id
                                .as_ref()
                                .map_or(false, |cid| !cid.value().is_empty())
                        });
                        if has_vat {
                            Ok(())
                        } else {
                            Err("SE supplier must have VAT number in PartyTaxScheme".into())
                        }
                    }
                }
            })
        },
    });

    // ── SE-R-002: SE VAT format check ─────────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-SE-R-002".into(),
        description: "SE VAT number must be 12 digits (SE + 10 digits)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "SE") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Ok(()),
                    Some(party) => {
                        for pts in &party.party_tax_scheme {
                            if let Some(cid) = &pts.company_id {
                                let v = cid.value();
                                if !v.is_empty() {
                                    // SE VAT: "SE" prefix followed by 10 digits, total 12 chars
                                    let valid = v.len() == 12
                                        && v.starts_with("SE")
                                        && v[2..].chars().all(|c| c.is_ascii_digit());
                                    if !valid {
                                        return Err(format!(
                                            "SE VAT number '{}' is invalid — must be SE + 10 digits",
                                            v
                                        ));
                                    }
                                }
                            }
                        }
                        Ok(())
                    }
                }
            })
        },
    });

    // ── SE-R-003: SE supplier PartyLegalEntity required ───────────────
    engine.add_rule(Rule {
        id: "PEPPOL-SE-R-003".into(),
        description: "SE supplier must have PartyLegalEntity".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "SE") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("SE supplier party is missing".into()),
                    Some(party) => {
                        if party.party_legal_entity.is_empty() {
                            Err("SE supplier must have PartyLegalEntity".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ── SE-R-004: SE supplier PartyIdentification required ────────────
    engine.add_rule(Rule {
        id: "PEPPOL-SE-R-004".into(),
        description: "SE supplier must have PartyIdentification".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "SE") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("SE supplier party is missing".into()),
                    Some(party) => {
                        if party.party_identification.is_empty() {
                            Err("SE supplier must have PartyIdentification".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ── SE-R-005: SE supplier postal address required ────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-SE-R-005".into(),
        description: "SE supplier must have postal address".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "SE") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("SE supplier party is missing".into()),
                    Some(party) => {
                        if party.postal_address.is_none() {
                            Err("SE supplier must have a postal address".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ── SE-R-006: SE supplier city name required ──────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-SE-R-006".into(),
        description: "SE supplier must have city name".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "SE") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("SE supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("SE supplier postal address is missing".into()),
                        Some(addr) => {
                            if addr.city_name.is_none() {
                                Err("SE supplier must have city name".into())
                            } else {
                                Ok(())
                            }
                        }
                    },
                }
            })
        },
    });

    // ── SE-R-007: SE supplier postal zone required ────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-SE-R-007".into(),
        description: "SE supplier must have postal zone".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "SE") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("SE supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("SE supplier postal address is missing".into()),
                        Some(addr) => {
                            if addr.postal_zone.is_none() {
                                Err("SE supplier must have postal zone".into())
                            } else {
                                Ok(())
                            }
                        }
                    },
                }
            })
        },
    });

    // ── SE-R-008: SE postal zone format check ─────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-SE-R-008".into(),
        description: "SE postal zone must be 5 digits".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "SE") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Ok(()),
                    Some(party) => match &party.postal_address {
                        None => Ok(()),
                        Some(addr) => match &addr.postal_zone {
                            None => Ok(()),
                            Some(zone) => {
                                let v = zone.value();
                                // Swedish postal codes: 5 digits, may have space (e.g., "123 45")
                                let digits: String =
                                    v.chars().filter(|c| !c.is_whitespace()).collect();
                                if digits.len() != 5 || !digits.chars().all(|c| c.is_ascii_digit())
                                {
                                    Err(format!(
                                        "SE postal zone '{}' is invalid — must be 5 digits",
                                        v
                                    ))
                                } else {
                                    Ok(())
                                }
                            }
                        },
                    },
                }
            })
        },
    });

    // ── SE-R-009: SE supplier contact required ────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-SE-R-009".into(),
        description: "SE supplier should have contact information".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "SE") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("SE supplier party is missing".into()),
                    Some(party) => {
                        if party.contact.is_none() {
                            Err("SE supplier should have contact information".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ── SE-R-010: SE supplier country code must be SE ─────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-SE-R-010".into(),
        description: "SE supplier country code must be SE".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "SE") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("SE supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("SE supplier postal address is missing".into()),
                        Some(addr) => match &addr.country {
                            None => Err("SE supplier country is missing".into()),
                            Some(country) => match &country.identification_code {
                                None => Err("SE supplier country code is missing".into()),
                                Some(code) => {
                                    if code.value() != "SE" {
                                        Err(format!(
                                            "SE supplier country code is '{}' — must be SE",
                                            code.value()
                                        ))
                                    } else {
                                        Ok(())
                                    }
                                }
                            },
                        },
                    },
                }
            })
        },
    });

    // ── SE-R-011: SE invoice must have tax total ──────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-SE-R-011".into(),
        description: "SE invoice must have at least one TaxTotal".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "SE") {
                    return Ok(());
                }
                if inv.tax_total.is_empty() {
                    Err("SE invoice must have at least one TaxTotal".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── SE-R-012: SE tax category must be S, Z, E, or AA ─────────────
    engine.add_rule(Rule {
        id: "PEPPOL-SE-R-012".into(),
        description: "SE tax categories must be S, Z, E, or AA".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "SE") {
                    return Ok(());
                }
                for tt in &inv.tax_total {
                    for st in &tt.tax_subtotal {
                        let code = st
                            .tax_category
                            .id
                            .as_ref()
                            .map(|id| id.value().to_string())
                            .unwrap_or_default();
                        if !["S", "Z", "E", "AA"].contains(&code.as_str()) {
                            return Err(format!(
                                "SE tax category '{}' not standard — expected S, Z, E, or AA",
                                code
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── SE-R-013: SE payment means reference required for credit transfers
    engine.add_rule(Rule {
        id: "PEPPOL-SE-R-013".into(),
        description: "SE payment means should be specified".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "SE") {
                    return Ok(());
                }
                // Informational: SE encourages explicit payment means
                if inv.payment_means.is_empty() {
                    Err("SE invoice should specify payment means".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════════════
    // GREECE (GR) — GR-R-001-1 through GR-R-010, GR-S-008-1, GR-S-011
    // ═══════════════════════════════════════════════════════════════════════

    // ── GR-R-001-1: GR supplier VAT required ─────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-GR-R-001-1".into(),
        description: "GR supplier must have VAT number (ΑΦΜ)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "GR") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("GR supplier party is missing".into()),
                    Some(party) => {
                        let has_vat = party.party_tax_scheme.iter().any(|pts| {
                            pts.company_id
                                .as_ref()
                                .map_or(false, |cid| !cid.value().is_empty())
                        });
                        if has_vat {
                            Ok(())
                        } else {
                            Err("GR supplier must have VAT number (ΑΦΜ) in PartyTaxScheme".into())
                        }
                    }
                }
            })
        },
    });

    // ── GR-R-002: GR VAT format check ──────────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-GR-R-002".into(),
        description: "GR VAT number must be 9 digits".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "GR") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Ok(()),
                    Some(party) => {
                        for pts in &party.party_tax_scheme {
                            if let Some(cid) = &pts.company_id {
                                let v = cid.value();
                                if !v.is_empty() {
                                    if v.len() != 9 || !v.chars().all(|c| c.is_ascii_digit()) {
                                        return Err(format!(
                                            "GR VAT number '{}' is invalid — must be 9 digits",
                                            v
                                        ));
                                    }
                                }
                            }
                        }
                        Ok(())
                    }
                }
            })
        },
    });

    // ── GR-R-003: GR supplier PartyIdentification required ──────────────
    engine.add_rule(Rule {
        id: "PEPPOL-GR-R-003".into(),
        description: "GR supplier must have PartyIdentification".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "GR") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("GR supplier party is missing".into()),
                    Some(party) => {
                        if party.party_identification.is_empty() {
                            Err("GR supplier must have PartyIdentification".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ── GR-R-004: GR invoice type code must be valid ──────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-GR-R-004".into(),
        description: "GR invoice must have valid InvoiceTypeCode".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "GR") {
                    return Ok(());
                }
                match &inv.invoice_type_code {
                    None => Err("GR invoice must have InvoiceTypeCode".into()),
                    Some(tc) => {
                        let code = tc.value();
                        // Greek MyData invoice types
                        let valid = matches!(
                            code,
                            "1.1"
                                | "1.2"
                                | "1.3"
                                | "1.4"
                                | "1.5"
                                | "1.6"
                                | "2.1"
                                | "2.2"
                                | "2.3"
                                | "2.4"
                                | "5.1"
                                | "5.2"
                                | "380"
                                | "381"
                                | "383"
                                | "384"
                                | "385"
                                | "386"
                                | "388"
                        );
                        if valid {
                            Ok(())
                        } else {
                            Err(format!(
                                "GR InvoiceTypeCode '{}' is not a valid MyData type",
                                code
                            ))
                        }
                    }
                }
            })
        },
    });

    // ── GR-R-005: GR supplier postal address required ──────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-GR-R-005".into(),
        description: "GR supplier must have postal address".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "GR") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("GR supplier party is missing".into()),
                    Some(party) => {
                        if party.postal_address.is_none() {
                            Err("GR supplier must have a postal address".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ── GR-R-006: GR supplier city name required ──────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-GR-R-006".into(),
        description: "GR supplier must have city name".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "GR") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("GR supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("GR supplier postal address is missing".into()),
                        Some(addr) => {
                            if addr.city_name.is_none() {
                                Err("GR supplier must have city name".into())
                            } else {
                                Ok(())
                            }
                        }
                    },
                }
            })
        },
    });

    // ── GR-R-007: GR supplier postal zone required ────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-GR-R-007".into(),
        description: "GR supplier must have postal zone".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "GR") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("GR supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("GR supplier postal address is missing".into()),
                        Some(addr) => {
                            if addr.postal_zone.is_none() {
                                Err("GR supplier must have postal zone".into())
                            } else {
                                Ok(())
                            }
                        }
                    },
                }
            })
        },
    });

    // ── GR-R-008: GR postal zone format check ─────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-GR-R-008".into(),
        description: "GR postal zone must be 5 digits".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "GR") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Ok(()),
                    Some(party) => match &party.postal_address {
                        None => Ok(()),
                        Some(addr) => match &addr.postal_zone {
                            None => Ok(()),
                            Some(zone) => {
                                let v = zone.value();
                                if v.len() != 5 || !v.chars().all(|c| c.is_ascii_digit()) {
                                    Err(format!(
                                        "GR postal zone '{}' is invalid — must be 5 digits",
                                        v
                                    ))
                                } else {
                                    Ok(())
                                }
                            }
                        },
                    },
                }
            })
        },
    });

    // ── GR-R-009: GR supplier contact required ────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-GR-R-009".into(),
        description: "GR supplier should have contact information".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "GR") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("GR supplier party is missing".into()),
                    Some(party) => {
                        if party.contact.is_none() {
                            Err("GR supplier should have contact information".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ── GR-R-010: GR tax total required ──────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-GR-R-010".into(),
        description: "GR invoice must have at least one TaxTotal".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "GR") {
                    return Ok(());
                }
                if inv.tax_total.is_empty() {
                    Err("GR invoice must have at least one TaxTotal".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── GR-S-008-1: GR supplier country code must be GR ──────────────
    engine.add_rule(Rule {
        id: "PEPPOL-GR-S-008-1".into(),
        description: "GR supplier country code must be GR".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "GR") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("GR supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("GR supplier postal address is missing".into()),
                        Some(addr) => match &addr.country {
                            None => Err("GR supplier country is missing".into()),
                            Some(country) => match &country.identification_code {
                                None => Err("GR supplier country code is missing".into()),
                                Some(code) => {
                                    if code.value() != "GR" {
                                        Err(format!(
                                            "GR supplier country code is '{}' — must be GR",
                                            code.value()
                                        ))
                                    } else {
                                        Ok(())
                                    }
                                }
                            },
                        },
                    },
                }
            })
        },
    });

    // ── GR-S-011: GR invoice must have MyData classification ─────────
    engine.add_rule(Rule {
        id: "PEPPOL-GR-S-011".into(),
        description: "GR invoice must include MyData classification fields".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "GR") {
                    return Ok(());
                }
                // MyData requires classification on invoice lines
                let mut missing_classification = false;
                for (i, line) in inv.invoice_line.iter().enumerate() {
                    if line.item.commodity_classification.is_empty() {
                        missing_classification = true;
                        // warn but continue to collect all
                        let _ = i; // suppress unused warning
                    }
                }
                if missing_classification {
                    Err("GR invoice lines should have MyData classification (CPV/HS codes)".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════════════
    // ICELAND (IS) — IS-R-001 through IS-R-010
    // ═══════════════════════════════════════════════════════════════════════

    // ── IS-R-001: IS supplier VAT required ────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IS-R-001".into(),
        description: "IS supplier must have VAT number (VSK)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IS") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("IS supplier party is missing".into()),
                    Some(party) => {
                        let has_vat = party.party_tax_scheme.iter().any(|pts| {
                            pts.company_id
                                .as_ref()
                                .map_or(false, |cid| !cid.value().is_empty())
                        });
                        if has_vat {
                            Ok(())
                        } else {
                            Err("IS supplier must have VAT number (VSK) in PartyTaxScheme".into())
                        }
                    }
                }
            })
        },
    });

    // ── IS-R-002: IS VAT format check ─────────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IS-R-002".into(),
        description: "IS VAT number format must be valid".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IS") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Ok(()),
                    Some(party) => {
                        for pts in &party.party_tax_scheme {
                            if let Some(cid) = &pts.company_id {
                                let v = cid.value();
                                if !v.is_empty() {
                                    // Icelandic VAT: 5 or 6 digits
                                    let digits_only: String =
                                        v.chars().filter(|c| !c.is_whitespace()).collect();
                                    if digits_only.len() < 5
                                        || digits_only.len() > 6
                                        || !digits_only.chars().all(|c| c.is_ascii_digit())
                                    {
                                        return Err(format!(
                                            "IS VAT number '{}' is invalid — must be 5-6 digits",
                                            v
                                        ));
                                    }
                                }
                            }
                        }
                        Ok(())
                    }
                }
            })
        },
    });

    // ── IS-R-003: IS supplier PartyIdentification required ────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IS-R-003".into(),
        description: "IS supplier must have PartyIdentification (Kennitala)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IS") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("IS supplier party is missing".into()),
                    Some(party) => {
                        if party.party_identification.is_empty() {
                            Err("IS supplier must have PartyIdentification (Kennitala)".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ── IS-R-004: IS supplier postal address required ────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IS-R-004".into(),
        description: "IS supplier must have postal address".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IS") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("IS supplier party is missing".into()),
                    Some(party) => {
                        if party.postal_address.is_none() {
                            Err("IS supplier must have a postal address".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ── IS-R-005: IS supplier city name required ──────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IS-R-005".into(),
        description: "IS supplier must have city name".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IS") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("IS supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("IS supplier postal address is missing".into()),
                        Some(addr) => {
                            if addr.city_name.is_none() {
                                Err("IS supplier must have city name".into())
                            } else {
                                Ok(())
                            }
                        }
                    },
                }
            })
        },
    });

    // ── IS-R-006: IS supplier postal zone required ────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IS-R-006".into(),
        description: "IS supplier must have postal zone".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IS") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("IS supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("IS supplier postal address is missing".into()),
                        Some(addr) => {
                            if addr.postal_zone.is_none() {
                                Err("IS supplier must have postal zone".into())
                            } else {
                                Ok(())
                            }
                        }
                    },
                }
            })
        },
    });

    // ── IS-R-007: IS postal zone format check ─────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IS-R-007".into(),
        description: "IS postal zone must be 3 digits".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IS") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Ok(()),
                    Some(party) => match &party.postal_address {
                        None => Ok(()),
                        Some(addr) => match &addr.postal_zone {
                            None => Ok(()),
                            Some(zone) => {
                                let v = zone.value();
                                if v.len() != 3 || !v.chars().all(|c| c.is_ascii_digit()) {
                                    Err(format!(
                                        "IS postal zone '{}' is invalid — must be 3 digits",
                                        v
                                    ))
                                } else {
                                    Ok(())
                                }
                            }
                        },
                    },
                }
            })
        },
    });

    // ── IS-R-008: IS supplier country code must be IS ─────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IS-R-008".into(),
        description: "IS supplier country code must be IS".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IS") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("IS supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("IS supplier postal address is missing".into()),
                        Some(addr) => match &addr.country {
                            None => Err("IS supplier country is missing".into()),
                            Some(country) => match &country.identification_code {
                                None => Err("IS supplier country code is missing".into()),
                                Some(code) => {
                                    if code.value() != "IS" {
                                        Err(format!(
                                            "IS supplier country code is '{}' — must be IS",
                                            code.value()
                                        ))
                                    } else {
                                        Ok(())
                                    }
                                }
                            },
                        },
                    },
                }
            })
        },
    });

    // ── IS-R-009: IS tax total required ───────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IS-R-009".into(),
        description: "IS invoice must have at least one TaxTotal".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IS") {
                    return Ok(());
                }
                if inv.tax_total.is_empty() {
                    Err("IS invoice must have at least one TaxTotal".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── IS-R-010: IS supplier contact required ────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-IS-R-010".into(),
        description: "IS supplier should have contact information".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "IS") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("IS supplier party is missing".into()),
                    Some(party) => {
                        if party.contact.is_none() {
                            Err("IS supplier should have contact information".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════════════
    // NETHERLANDS (NL) — NL-R-001 through NL-R-009
    // ═══════════════════════════════════════════════════════════════════════

    // ── NL-R-001: NL supplier KVK number required ────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-NL-R-001".into(),
        description: "NL supplier must have KVK number in PartyLegalEntity".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "NL") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("NL supplier party is missing".into()),
                    Some(party) => {
                        let has_kvk = party.party_legal_entity.iter().any(|ple| {
                            ple.company_id
                                .as_ref()
                                .map_or(false, |cid| !cid.value().is_empty())
                        });
                        if has_kvk {
                            Ok(())
                        } else {
                            Err("NL supplier must have KVK number in PartyLegalEntity".into())
                        }
                    }
                }
            })
        },
    });

    // ── NL-R-002: NL KVK format check ─────────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-NL-R-002".into(),
        description: "NL KVK number must be 8 digits".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "NL") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Ok(()),
                    Some(party) => {
                        for ple in &party.party_legal_entity {
                            if let Some(cid) = &ple.company_id {
                                let v = cid.value();
                                if !v.is_empty() {
                                    if v.len() != 8 || !v.chars().all(|c| c.is_ascii_digit()) {
                                        return Err(format!(
                                            "NL KVK number '{}' is invalid — must be 8 digits",
                                            v
                                        ));
                                    }
                                }
                            }
                        }
                        Ok(())
                    }
                }
            })
        },
    });

    // ── NL-R-003: NL supplier VAT required ────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-NL-R-003".into(),
        description: "NL supplier must have VAT number (BTW)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "NL") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("NL supplier party is missing".into()),
                    Some(party) => {
                        let has_vat = party.party_tax_scheme.iter().any(|pts| {
                            pts.company_id
                                .as_ref()
                                .map_or(false, |cid| !cid.value().is_empty())
                        });
                        if has_vat {
                            Ok(())
                        } else {
                            Err("NL supplier must have VAT number (BTW) in PartyTaxScheme".into())
                        }
                    }
                }
            })
        },
    });

    // ── NL-R-004: NL VAT format check ─────────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-NL-R-004".into(),
        description: "NL VAT number format must be NL + 9 digits + B01-B99".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "NL") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Ok(()),
                    Some(party) => {
                        for pts in &party.party_tax_scheme {
                            if let Some(cid) = &pts.company_id {
                                let v = cid.value();
                                if !v.is_empty() {
                                    // NL VAT: "NL" + 9 digits + "B" + 2 digits, total 14
                                    let valid = v.len() == 14
                                        && v.starts_with("NL")
                                        && v[2..11].chars().all(|c| c.is_ascii_digit())
                                        && v[11..].starts_with('B')
                                        && v[12..].chars().all(|c| c.is_ascii_digit());
                                    if !valid {
                                        return Err(format!(
                                            "NL VAT number '{}' is invalid — must be NL + 9 digits + B + 2 digits",
                                            v
                                        ));
                                    }
                                }
                            }
                        }
                        Ok(())
                    }
                }
            })
        },
    });

    // ── NL-R-005: NL supplier postal address required ────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-NL-R-005".into(),
        description: "NL supplier must have postal address".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "NL") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("NL supplier party is missing".into()),
                    Some(party) => {
                        if party.postal_address.is_none() {
                            Err("NL supplier must have a postal address".into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })
        },
    });

    // ── NL-R-006: NL postal zone format check ────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-NL-R-006".into(),
        description: "NL postal zone must be 4 digits + 2 letters".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "NL") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Ok(()),
                    Some(party) => match &party.postal_address {
                        None => Ok(()),
                        Some(addr) => match &addr.postal_zone {
                            None => Ok(()),
                            Some(zone) => {
                                let v = zone.value();
                                // NL postal code: 4 digits + 2 letters (e.g., "1234AB")
                                let compact: String = v
                                    .chars()
                                    .filter(|c| !c.is_whitespace())
                                    .collect();
                                let valid = compact.len() == 6
                                    && compact[..4].chars().all(|c| c.is_ascii_digit())
                                    && compact[4..].chars().all(|c| c.is_ascii_alphabetic());
                                if !valid {
                                    Err(format!(
                                        "NL postal zone '{}' is invalid — must be 4 digits + 2 letters",
                                        v
                                    ))
                                } else {
                                    Ok(())
                                }
                            }
                        },
                    },
                }
            })
        },
    });

    // ── NL-R-007: NL supplier city name required ──────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-NL-R-007".into(),
        description: "NL supplier must have city name".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "NL") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("NL supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("NL supplier postal address is missing".into()),
                        Some(addr) => {
                            if addr.city_name.is_none() {
                                Err("NL supplier must have city name".into())
                            } else {
                                Ok(())
                            }
                        }
                    },
                }
            })
        },
    });

    // ── NL-R-008: NL payment means required ──────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-NL-R-008".into(),
        description: "NL invoice should specify payment means".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "NL") {
                    return Ok(());
                }
                if inv.payment_means.is_empty() {
                    Err("NL invoice should specify payment means".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── NL-R-009: NL supplier country code must be NL ─────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-NL-R-009".into(),
        description: "NL supplier country code must be NL".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if !supplier_country_is(&inv, "NL") {
                    return Ok(());
                }
                match &inv.accounting_supplier_party.party {
                    None => Err("NL supplier party is missing".into()),
                    Some(party) => match &party.postal_address {
                        None => Err("NL supplier postal address is missing".into()),
                        Some(addr) => match &addr.country {
                            None => Err("NL supplier country is missing".into()),
                            Some(country) => match &country.identification_code {
                                None => Err("NL supplier country code is missing".into()),
                                Some(code) => {
                                    if code.value() != "NL" {
                                        Err(format!(
                                            "NL supplier country code is '{}' — must be NL",
                                            code.value()
                                        ))
                                    } else {
                                        Ok(())
                                    }
                                }
                            },
                        },
                    },
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
    use rust_decimal::Decimal;
    use std::sync::Arc;
    use ubl_common::cac::address::{Country, PostalAddress};
    use ubl_common::cac::party::{Party, PartyIdentification, PartyLegalEntity, PartyTaxScheme};
    use ubl_common::cac::tax::TaxScheme;
    use ubl_common::cbc;
    use ubl_common::cbc::*;
    use ubl_documents::billing::Invoice;

    fn supplier_party_with_country(iso: &str) -> Party {
        Party {
            mark_care_indicator: None,
            mark_attention_indicator: None,
            website_uri: None,
            logo_reference_id: None,
            endpoint_id: None,
            industry_classification_code: None,
            party_identification: vec![],
            party_name: vec![],
            language: None,
            postal_address: Some(PostalAddress {
                id: None,
                address_format_code: None,
                address_type_code: None,
                block_name: None,
                building_name: None,
                building_number: None,
                city_name: None,
                city_subdivision_name: None,
                country_subentity: None,
                country_subentity_code: None,
                department: None,
                description: None,
                district: None,
                floor: None,
                inhouse_mail: None,
                mark_attention: None,
                mark_care: None,
                plot_identification: None,
                postal_zone: None,
                postbox: None,
                region: None,
                room: None,
                street_name: None,
                additional_street_name: None,
                country: Some(Country {
                    identification_code: Some(CountryCode::new(iso)),
                    name: None,
                }),
                timezone_offset: None,
                address_line: vec![],
            }),
            physical_location: None,
            party_tax_scheme: vec![],
            party_legal_entity: vec![],
            contact: None,
            person: None,
            agent_party: None,
        }
    }

    fn minimal_invoice() -> Invoice {
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

    fn add_dk_supplier(inv: &mut Invoice) {
        inv.accounting_supplier_party.party = Some(Party {
            party_legal_entity: vec![PartyLegalEntity {
                registration_name: Some(RegistrationName::new("Danish Company A/S")),
                company_id: Some(CompanyID::new("12345678")),
                registration_date: None,
                registration_expiration_date: None,
                company_legal_form: None,
                company_legal_form_code: None,
                sole_proprietorship_indicator: None,
                corporate_stock_amount: None,
                fully_paid_shares_indicator: None,
                company_liquidation_status_code: None,
                corporate_registration_type_code: None,
                entity_size_code: None,
            }],
            party_tax_scheme: vec![PartyTaxScheme {
                registration_name: Some(RegistrationName::new("Danish Company A/S")),
                company_id: Some(CompanyID::new("DK12345678")),
                tax_level_code: None,
                exemption_reason_code: None,
                exemption_reason: None,
                tax_scheme: TaxScheme {
                    id: Some(ID::new("VAT")),
                    name: None,
                    tax_type_code: None,
                    currency_code: None,
                    jurisdiction_region_address: vec![],
                },
            }],
            ..supplier_party_with_country("DK")
        });
    }

    fn add_it_supplier(inv: &mut Invoice) {
        inv.accounting_supplier_party.party = Some(Party {
            party_identification: vec![PartyIdentification {
                id: ID::new("RSSMRA80A01H501A"),
            }],
            party_tax_scheme: vec![PartyTaxScheme {
                registration_name: Some(RegistrationName::new("Italian Company Srl")),
                company_id: Some(CompanyID::new("12345678901")),
                tax_level_code: None,
                exemption_reason_code: None,
                exemption_reason: None,
                tax_scheme: TaxScheme {
                    id: Some(ID::new("VAT")),
                    name: None,
                    tax_type_code: None,
                    currency_code: None,
                    jurisdiction_region_address: vec![],
                },
            }],
            ..supplier_party_with_country("IT")
        });
    }

    // ─── Denmark tests ───

    #[test]
    fn test_dk_r002_cvr_missing_fails() {
        let mut inv = minimal_invoice();
        inv.accounting_supplier_party.party = Some(supplier_party_with_country("DK"));
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(
            failures
                .iter()
                .any(|f| f.rule_id == "PEPPOL-DK-R-002" && f.severity == Some(Severity::Fatal))
        );
    }

    #[test]
    fn test_dk_r002_cvr_present_passes() {
        let mut inv = minimal_invoice();
        add_dk_supplier(&mut inv);
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(!failures.iter().any(|f| f.rule_id == "PEPPOL-DK-R-002"));
    }

    #[test]
    fn test_dk_r003_cvr_bad_format_fails() {
        let mut inv = minimal_invoice();
        let mut party = supplier_party_with_country("DK");
        party.party_legal_entity = vec![PartyLegalEntity {
            registration_name: None,
            company_id: Some(CompanyID::new("1234")),
            registration_date: None,
            registration_expiration_date: None,
            company_legal_form: None,
            company_legal_form_code: None,
            sole_proprietorship_indicator: None,
            corporate_stock_amount: None,
            fully_paid_shares_indicator: None,
            company_liquidation_status_code: None,
            corporate_registration_type_code: None,
            entity_size_code: None,
        }];
        inv.accounting_supplier_party.party = Some(party);
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-DK-R-003"));
    }

    #[test]
    fn test_non_dk_skips_dk_rules() {
        let mut inv = minimal_invoice();
        inv.accounting_supplier_party.party = Some(supplier_party_with_country("FR"));
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        // No DK rules should fire for a French supplier
        assert!(!failures.iter().any(|f| f.rule_id.starts_with("PEPPOL-DK-")));
    }

    // ─── Italy tests ───

    #[test]
    fn test_it_r001_vat_missing_fails() {
        let mut inv = minimal_invoice();
        inv.accounting_supplier_party.party = Some(supplier_party_with_country("IT"));
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-IT-R-001"));
    }

    #[test]
    fn test_it_r001_vat_present_passes() {
        let mut inv = minimal_invoice();
        add_it_supplier(&mut inv);
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(!failures.iter().any(|f| f.rule_id == "PEPPOL-IT-R-001"));
    }

    // ─── Norway tests ───

    #[test]
    fn test_no_r001_orgnr_missing_fails() {
        let mut inv = minimal_invoice();
        inv.accounting_supplier_party.party = Some(supplier_party_with_country("NO"));
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "PEPPOL-NO-R-001"));
    }

    #[test]
    fn test_no_r001_orgnr_present_passes() {
        let mut inv = minimal_invoice();
        let mut party = supplier_party_with_country("NO");
        party.party_legal_entity = vec![PartyLegalEntity {
            registration_name: Some(RegistrationName::new("Norsk Bedrift AS")),
            company_id: Some(CompanyID::new("123456789")),
            registration_date: None,
            registration_expiration_date: None,
            company_legal_form: None,
            company_legal_form_code: None,
            sole_proprietorship_indicator: None,
            corporate_stock_amount: None,
            fully_paid_shares_indicator: None,
            company_liquidation_status_code: None,
            corporate_registration_type_code: None,
            entity_size_code: None,
        }];
        inv.accounting_supplier_party.party = Some(party);
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(inv));
        let failures = engine.evaluate_failures();
        assert!(!failures.iter().any(|f| f.rule_id == "PEPPOL-NO-R-001"));
    }
}
