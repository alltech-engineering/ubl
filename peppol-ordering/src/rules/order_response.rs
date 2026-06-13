// Peppol BIS Ordering 3.0 — OrderResponse Validation Rules
//
// Validates OrderResponse documents according to Peppol BIS Ordering.
// Rules ORESP-R001 through ORESP-R011.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::OrderResponse;

/// Returns true if the OrderResponseCode value indicates acceptance.
fn is_acceptance(code: &str) -> bool {
    matches!(code, "1" | "CA" | "6" | "accepted" | "Accepted")
        || code.to_lowercase().contains("accept")
}

/// Returns true if the OrderResponseCode value indicates rejection or change.
fn is_rejection_or_change(code: &str) -> bool {
    matches!(
        code,
        "2" | "RE" | "3" | "CH" | "rejected" | "Rejected" | "changed" | "Changed"
    ) || code.to_lowercase().contains("reject")
        || code.to_lowercase().contains("change")
}

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<OrderResponse>) {
    // ── ORESP-R001 (Fatal): OrderResponse ID must be present ───────────────
    engine.add_rule(Rule {
        id: "ORESP-R001".into(),
        description: "OrderResponse ID must be present and non-empty".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.id.value().is_empty() {
                    Err("OrderResponse ID is empty — a non-empty identifier is required".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORESP-R002 (Fatal): Issue date must be present ────────────────────
    engine.add_rule(Rule {
        id: "ORESP-R002".into(),
        description: "Issue date must be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                let _date = &inv.issue_date;
                Ok(())
            })
        },
    });

    // ── ORESP-R003 (Fatal): Must reference original Order ─────────────────
    //    Checks both order_reference (cac:OrderReference) and
    //    order_document_reference (cac:OrderDocumentReference) for an ID.
    engine.add_rule(Rule {
        id: "ORESP-R003".into(),
        description: "Must reference the original Order via OrderReference or OrderDocumentReference with ID".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                let has_order_ref = inv.order_reference.iter().any(|r| {
                    r.id.as_ref().map(|id| !id.value().is_empty()).unwrap_or(false)
                });
                let has_doc_ref = inv.order_document_reference.iter().any(|dr| {
                    dr.id.as_ref().map(|id| !id.value().is_empty()).unwrap_or(false)
                });
                if has_order_ref || has_doc_ref {
                    Ok(())
                } else {
                    Err("No reference to the original Order — an OrderReference or OrderDocumentReference with an ID is required".into())
                }
            })
        },
    });

    // ── ORESP-R004 (Fatal): Seller party must be present ──────────────────
    engine.add_rule(Rule {
        id: "ORESP-R004".into(),
        description: "Seller party must be present (the responder)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.seller_supplier_party.party {
                None => Err("Seller party is missing — the responding party is required".into()),
                Some(_) => Ok(()),
            })
        },
    });

    // ── ORESP-R005 (Fatal): Buyer party must be present ───────────────────
    engine.add_rule(Rule {
        id: "ORESP-R005".into(),
        description: "Buyer party must be present (the original order sender)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.buyer_customer_party.party {
                None => {
                    Err("Buyer party is missing — the original order sender is required".into())
                }
                Some(_) => Ok(()),
            })
        },
    });

    // ── ORESP-R006 (Fatal): Response code must be present ─────────────────
    engine.add_rule(Rule {
        id: "ORESP-R006".into(),
        description: "Response code must be present (OrderResponseCode)".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.order_response_code {
                None => Err("OrderResponseCode is missing — a response code is required".into()),
                Some(code) if code.value().is_empty() => {
                    Err("OrderResponseCode is present but empty".into())
                }
                Some(_) => Ok(()),
            })
        },
    });

    // ── ORESP-R007 (Warning): Notes should explain changes/rejections ─────
    engine.add_rule(Rule {
        id: "ORESP-R007".into(),
        description: "Notes should explain any changes or rejections".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.note.is_empty() {
                    Err("No notes provided — consider adding explanation of the response".into())
                } else if inv.note.iter().all(|n| n.value().trim().is_empty()) {
                    Err(
                        "Notes are present but all are empty — provide meaningful explanation"
                            .into(),
                    )
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── ORESP-R008 (Fatal): Accepted orders must have lines ───────────────
    engine.add_rule(Rule {
        id: "ORESP-R008".into(),
        description: "If order is accepted, at least one OrderLine should be present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.order_response_code {
                Some(code) if is_acceptance(code.value()) => {
                    if inv.order_line.is_empty() {
                        Err("OrderResponse indicates acceptance but no OrderLines are present — at least one accepted line is required".into())
                    } else {
                        Ok(())
                    }
                }
                _ => {
                    // Not an acceptance; lines may be empty for rejections
                    Ok(())
                }
            })
        },
    });

    // ── ORESP-R009 (Warning): Rejection/changes should document reason ────
    engine.add_rule(Rule {
        id: "ORESP-R009".into(),
        description: "If order is rejected or changed, reason should be documented".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || match &inv.order_response_code {
                Some(code) if is_rejection_or_change(code.value()) => {
                    if inv.note.is_empty() || inv.note.iter().all(|n| n.value().trim().is_empty()) {
                        Err("OrderResponse indicates rejection or change but no meaningful notes document the reason".into())
                    } else {
                        Ok(())
                    }
                }
                _ => Ok(()),
            })
        },
    });

    // ── ORESP-R010 (Fatal): Seller party identification must be present ───
    engine.add_rule(Rule {
        id: "ORESP-R010".into(),
        description: "Seller party identification must be present".into(),
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

    // ── ORESP-R011 (Fatal): Buyer party identification must be present ────
    engine.add_rule(Rule {
        id: "ORESP-R011".into(),
        description: "Buyer party identification must be present".into(),
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
}
