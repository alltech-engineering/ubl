// Peppol BIS Billing 3.0 — Party Business Rules
//
// Validates supplier (AccountingSupplierParty) and customer (AccountingCustomerParty)
// party information per EN16931 / Peppol BIS Billing 3.0.
// Rules reference: EN16931 Schematron and Peppol BIS Billing 3.0 specification.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::billing::Invoice;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Invoice>) {
    // ═══════════════════════════════════════════════════════════════
    // SUPPLIER RULES  (AccountingSupplierParty)
    // ═══════════════════════════════════════════════════════════════

    // ── BT023: Seller name must be present ───────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-BT023".into(),
        description: "Seller name must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.accounting_supplier_party.party {
                None => Err("Supplier party is missing — seller name cannot be verified".into()),
                Some(party) => {
                    if party.party_name.is_empty() {
                        Err("Supplier party name is empty — a seller name is required".into())
                    } else {
                        // party_name vec has at least one entry
                        Ok(())
                    }
                }
            })
        },
    });

    // ── R020: Supplier must have PartyIdentification ─────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R020".into(),
        description: "Supplier must have PartyIdentification".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.accounting_supplier_party.party {
                None => Err("Supplier party is missing — party identification cannot be verified".into()),
                Some(party) => {
                    if party.party_identification.is_empty() {
                        Err("Supplier has no PartyIdentification — at least one identifier is required".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ── R040: Supplier country code must be ISO 3166-1 alpha-2 ──
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R040".into(),
        description: "Supplier country code must be ISO 3166-1 alpha-2 (exactly 2 chars)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.accounting_supplier_party.party {
                None => Err("Supplier party is missing — country code cannot be verified".into()),
                Some(party) => match &party.postal_address {
                    None => Err("Supplier postal address is missing — country code cannot be verified".into()),
                    Some(addr) => match &addr.country {
                        None => Err("Supplier country is missing — country code cannot be verified".into()),
                        Some(country) => match &country.identification_code {
                            None => Err("Supplier country identification code is missing".into()),
                            Some(code) => {
                                if code.value().len() != 2 {
                                    Err(format!(
                                        "Supplier country code '{}' is not valid ISO 3166-1 alpha-2 (must be exactly 2 characters)",
                                        code.value()
                                    ))
                                } else {
                                    Ok(())
                                }
                            }
                        },
                    },
                },
            })
        },
    });

    // ── R041: Supplier city name should be present ───────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R041".into(),
        description: "Supplier city name should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.accounting_supplier_party.party {
                None => Err("Supplier party is missing — city name cannot be verified".into()),
                Some(party) => match &party.postal_address {
                    None => Err("Supplier postal address is missing".into()),
                    Some(addr) => {
                        if addr.city_name.is_none() {
                            Err("Supplier city name is not present — should be provided".into())
                        } else {
                            Ok(())
                        }
                    }
                },
            })
        },
    });

    // ── R042: Supplier street name should be present ─────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R042".into(),
        description: "Supplier street name should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.accounting_supplier_party.party {
                None => Err("Supplier party is missing — street name cannot be verified".into()),
                Some(party) => match &party.postal_address {
                    None => Err("Supplier postal address is missing".into()),
                    Some(addr) => {
                        if addr.street_name.is_none() {
                            Err("Supplier street name is not present — should be provided".into())
                        } else {
                            Ok(())
                        }
                    }
                },
            })
        },
    });

    // ── R043: Supplier contact should be present ─────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R043".into(),
        description: "Supplier contact should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.accounting_supplier_party.party {
                None => Err("Supplier party is missing — contact cannot be verified".into()),
                Some(party) => {
                    if party.contact.is_none() {
                        Err("Supplier contact is not present — should be provided".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ── R051: Supplier VAT number in PartyTaxScheme should be present ─
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R051".into(),
        description: "Supplier VAT number in PartyTaxScheme should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.accounting_supplier_party.party {
                None => Err("Supplier party is missing — VAT number cannot be verified".into()),
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
                            "Supplier has no VAT number in PartyTaxScheme — should be provided"
                                .into(),
                        )
                    }
                }
            })
        },
    });

    // ── R061: Supplier legal entity registration should be present ─
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R061".into(),
        description: "Supplier legal entity registration should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.accounting_supplier_party.party {
                None => Err("Supplier party is missing — legal entity cannot be verified".into()),
                Some(party) => {
                    if party.party_legal_entity.is_empty() {
                        Err(
                            "Supplier legal entity registration is missing — should be provided"
                                .into(),
                        )
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ── R110: PartyTaxScheme TaxScheme must have ID = "VAT" ──────
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R110".into(),
        description: "PartyTaxScheme TaxScheme must have ID = \"VAT\"".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.accounting_supplier_party.party {
                None => Err("Supplier party is missing — tax scheme ID cannot be verified".into()),
                Some(party) => {
                    for pts in &party.party_tax_scheme {
                        match &pts.tax_scheme.id {
                            None => {
                                return Err(
                                    "PartyTaxScheme TaxScheme ID is missing — must be \"VAT\""
                                        .into(),
                                );
                            }
                            Some(id) if id.value() != "VAT" => {
                                return Err(format!(
                                    "PartyTaxScheme TaxScheme ID is '{}' — must be \"VAT\"",
                                    id.value()
                                ));
                            }
                            Some(_) => { /* OK */ }
                        }
                    }
                    Ok(())
                }
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════
    // CUSTOMER RULES  (AccountingCustomerParty)
    // ═══════════════════════════════════════════════════════════════

    // ── R010: Customer party must have identification if present ─
    engine.add_rule(Rule {
        id: "PEPPOL-EN16931-R010".into(),
        description: "Customer party must have identification if present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.accounting_customer_party {
                None => Ok(()),
                Some(customer) => match &customer.party {
                    None => Ok(()),
                    Some(party) => {
                        if party.party_identification.is_empty() {
                            Err(
                                "Customer party is present but has no PartyIdentification — at least one identifier is required"
                                    .into(),
                            )
                        } else {
                            Ok(())
                        }
                    }
                },
            })
        },
    });
}
