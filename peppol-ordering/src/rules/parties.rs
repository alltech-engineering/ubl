// Peppol BIS Ordering 3.0 — Party Business Rules
//
// Validates buyer (BuyerCustomerParty) and seller (SellerSupplierParty)
// party information for Purchase Orders.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Order>) {
    // ═══════════════════════════════════════════════════════════════
    // BUYER RULES  (BuyerCustomerParty)
    // ═══════════════════════════════════════════════════════════════

    // ── ORD-R010 (Fatal): Buyer party must be present and have name ───────
    engine.add_rule(Rule {
        id: "ORD-R010".into(),
        description: "Buyer party must be present and have name".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — buyer name cannot be verified".into()),
                Some(party) => {
                    if party.party_name.is_empty() {
                        Err("Buyer party name is empty — a buyer name is required".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ── ORD-R011 (Fatal): Buyer must have party identification ────────────
    engine.add_rule(Rule {
        id: "ORD-R011".into(),
        description: "Buyer must have party identification".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — party identification cannot be verified".into()),
                Some(party) => {
                    if party.party_identification.is_empty() {
                        Err("Buyer has no PartyIdentification — at least one identifier is required".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ═══════════════════════════════════════════════════════════════
    // SELLER RULES  (SellerSupplierParty)
    // ═══════════════════════════════════════════════════════════════

    // ── ORD-R012 (Fatal): Seller party must be present and have name ──────
    engine.add_rule(Rule {
        id: "ORD-R012".into(),
        description: "Seller party must be present and have name".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — seller name cannot be verified".into()),
                Some(party) => {
                    if party.party_name.is_empty() {
                        Err("Seller party name is empty — a seller name is required".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ── ORD-R013 (Fatal): Seller must have party identification ───────────
    engine.add_rule(Rule {
        id: "ORD-R013".into(),
        description: "Seller must have party identification".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — party identification cannot be verified".into()),
                Some(party) => {
                    if party.party_identification.is_empty() {
                        Err("Seller has no PartyIdentification — at least one identifier is required".into())
                    } else {
                        Ok(())
                    }
                }
            })
        },
    });

    // ── ORD-R014 (Warning): Buyer postal address should include country ────
    engine.add_rule(Rule {
        id: "ORD-R014".into(),
        description: "Buyer postal address should include country".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => Err("Buyer party is missing — postal address cannot be verified".into()),
                Some(party) => match &party.postal_address {
                    None => Err("Buyer postal address is missing".into()),
                    Some(addr) => {
                        if addr.country.is_none() {
                            Err("Buyer postal address does not include a country".into())
                        } else {
                            Ok(())
                        }
                    }
                },
            })
        },
    });

    // ── ORD-R015 (Warning): Seller postal address should include country ───
    engine.add_rule(Rule {
        id: "ORD-R015".into(),
        description: "Seller postal address should include country".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — postal address cannot be verified".into()),
                Some(party) => match &party.postal_address {
                    None => Err("Seller postal address is missing".into()),
                    Some(addr) => {
                        if addr.country.is_none() {
                            Err("Seller postal address does not include a country".into())
                        } else {
                            Ok(())
                        }
                    }
                },
            })
        },
    });

    // ── ORD-R016 (Warning): Buyer contact should be present ───────────────
    engine.add_rule(Rule {
        id: "ORD-R016".into(),
        description: "Buyer contact should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.buyer_customer_party.buyer_contact.is_none() {
                    Err("Buyer contact is not present — should be provided".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORD-R017 (Warning): Seller contact should be present ──────────────
    engine.add_rule(Rule {
        id: "ORD-R017".into(),
        description: "Seller contact should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.seller_supplier_party.seller_contact.is_none() {
                    Err("Seller contact is not present — should be provided".into())
                } else {
                    Ok(())
                }
            })
        },
    });
}
