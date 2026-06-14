// Peppol BIS IMR 3.0 — Header & Document Rules
//
// Validates the ApplicationResponse (Invoice Message Response) per Peppol BIS IMR 3.0.
// IMR is the business-level response to an invoice — approve, reject, or conditionally accept.
//
// Reference: https://docs.peppol.eu/poacc/imr/3.0/

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::status::ApplicationResponse;

pub fn add_rules(engine: &mut RuleEngine, imr: &Arc<ApplicationResponse>) {
    // ── IMR-R001: IMR ID must be present (Fatal) ──────────────────────────
    engine.add_rule(Rule {
        id: "IMR-R001".into(),
        description: "IMR ID must be present and non-empty".into(),
        severity: Severity::Fatal,
        check: {
            let imr = Arc::clone(imr);
            Box::new(move || {
                let v = imr.id.value();
                if v.is_empty() {
                    Err("IMR ID is empty — a non-empty identifier is required".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── IMR-R002: Issue date must be present (Fatal) ──────────────────────
    engine.add_rule(Rule {
        id: "IMR-R002".into(),
        description: "IMR issue date must be present".into(),
        severity: Severity::Fatal,
        check: {
            let imr = Arc::clone(imr);
            Box::new(move || {
                // issue_date is a required struct field — always present.
                // This rule passes trivially but is included for completeness.
                let _date = &imr.issue_date;
                Ok(())
            })
        },
    });

    // ── IMR-R003: Must reference the invoice being responded to (Fatal) ───
    engine.add_rule(Rule {
        id: "IMR-R003".into(),
        description:
            "Must reference the invoice being responded to (DocumentReference.ID)".into(),
        severity: Severity::Fatal,
        check: {
            let imr = Arc::clone(imr);
            Box::new(move || {
                if imr.document_response.is_empty() {
                    return Err(
                        "No DocumentResponse present — must reference the invoice being responded to"
                            .into(),
                    );
                }
                let has_ref = imr.document_response.iter().any(|dr| {
                    dr.document_reference.iter().any(|dref| {
                        dref
                            .id
                            .as_ref()
                            .map(|id| !id.value().is_empty())
                            .unwrap_or(false)
                    })
                });
                if has_ref {
                    Ok(())
                } else {
                    Err(
                        "No DocumentReference with a non-empty ID found — must reference the invoice being responded to"
                            .into(),
                    )
                }
            })
        },
    });

    // ── IMR-R004: Response code must be present (Fatal) ───────────────────
    engine.add_rule(Rule {
        id: "IMR-R004".into(),
        description:
            "Response code must be present (approved/rejected/conditionally accepted)".into(),
        severity: Severity::Fatal,
        check: {
            let imr = Arc::clone(imr);
            Box::new(move || {
                if imr.document_response.is_empty() {
                    return Err(
                        "No DocumentResponse present — response code is required".into(),
                    );
                }
                let has_code = imr
                    .document_response
                    .iter()
                    .any(|dr| dr.response.response_code.is_some());
                if has_code {
                    Ok(())
                } else {
                    Err(
                        "Response.ResponseCode is missing — a response code (e.g., AP, RE, CA) is required"
                            .into(),
                    )
                }
            })
        },
    });

    // ── IMR-R005: Response description must explain the decision (Fatal) ───
    engine.add_rule(Rule {
        id: "IMR-R005".into(),
        description: "Response description must explain the decision".into(),
        severity: Severity::Fatal,
        check: {
            let imr = Arc::clone(imr);
            Box::new(move || {
                if imr.document_response.is_empty() {
                    return Err(
                        "No DocumentResponse with a description — must explain the decision"
                            .into(),
                    );
                }
                let has_desc = imr.document_response.iter().any(|dr| {
                    !dr.response.description.is_empty()
                        && dr
                            .response
                            .description
                            .iter()
                            .any(|d| !d.value().trim().is_empty())
                });
                if has_desc {
                    Ok(())
                } else {
                    Err(
                        "Response description is missing or empty — must explain the decision"
                            .into(),
                    )
                }
            })
        },
    });

    // ── IMR-R006: If rejected/conditionally accepted, reason must be in notes (Warning) ──
    engine.add_rule(Rule {
        id: "IMR-R006".into(),
        description:
            "If rejected or conditionally accepted, reason must be provided in notes".into(),
        severity: Severity::Warning,
        check: {
            let imr = Arc::clone(imr);
            Box::new(move || {
                // Determine if the response code indicates rejection or conditional acceptance
                let is_rejection_or_conditional = imr.document_response.iter().any(|dr| {
                    dr.response
                        .response_code
                        .as_ref()
                        .map(|rc| {
                            let code = rc.value();
                            code == "RE" || code == "CA"
                        })
                        .unwrap_or(false)
                });

                if !is_rejection_or_conditional {
                    // Rule does not apply — nothing to check
                    return Ok(());
                }

                // Check that notes are present and non-empty
                if imr.note.is_empty() {
                    Err(
                        "IMR rejected or conditionally accepted but no notes provided — reason must be given"
                            .into(),
                    )
                } else if imr.note.iter().all(|n| n.value().trim().is_empty()) {
                    Err(
                        "Notes are present but all are empty — a reason must be given for rejection or conditional acceptance"
                            .into(),
                    )
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── IMR-R007: Sender and receiver parties must be present (Fatal) ─────
    engine.add_rule(Rule {
        id: "IMR-R007".into(),
        description: "Sender and receiver parties must both be present".into(),
        severity: Severity::Fatal,
        check: {
            let imr = Arc::clone(imr);
            Box::new(move || {
                let sender_ok = imr.sender_party.party.is_some();
                let receiver_ok = imr.receiver_party.party.is_some();

                match (sender_ok, receiver_ok) {
                    (true, true) => Ok(()),
                    (false, false) => Err(
                        "Both sender and receiver parties are missing — the IMR must identify both parties"
                            .into(),
                    ),
                    (false, true) => Err(
                        "Sender party is missing — the IMR must identify who sent it".into(),
                    ),
                    (true, false) => Err(
                        "Receiver party is missing — the IMR must identify the recipient".into(),
                    ),
                }
            })
        },
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use peppol_common::rules::RuleEngine;
    use std::sync::Arc;
    use ubl_common::cbc;
    use ubl_common::cac;
    use ubl_documents::status::ApplicationResponse;

    fn minimal_imr_accepted() -> ApplicationResponse {
        use chrono::NaiveDate;
        ApplicationResponse {
            id: cbc::ID::new("IMR-001"),
            issue_date: cbc::IssueDate::new(NaiveDate::from_ymd_opt(2026, 6, 12).unwrap()),
            sender_party: ubl_documents::status::application_response::SenderParty {
                party: Some(cac::Party {
                    mark_care_indicator: None,
                    mark_attention_indicator: None,
                    website_uri: None,
                    logo_reference_id: None,
                    endpoint_id: None,
                    industry_classification_code: None,
                    party_identification: vec![],
                    party_name: vec![cac::PartyName {
                        name: cbc::Name::new("Beta Ltd"),
                    }],
                    language: None,
                    postal_address: None,
                    physical_location: None,
                    party_tax_scheme: vec![],
                    party_legal_entity: vec![],
                    contact: None,
                    person: None,
                    agent_party: None,
                }),
            },
            receiver_party: ubl_documents::status::application_response::ReceiverParty {
                party: Some(cac::Party {
                    mark_care_indicator: None,
                    mark_attention_indicator: None,
                    website_uri: None,
                    logo_reference_id: None,
                    endpoint_id: None,
                    industry_classification_code: None,
                    party_identification: vec![],
                    party_name: vec![cac::PartyName {
                        name: cbc::Name::new("Acme Corp"),
                    }],
                    language: None,
                    postal_address: None,
                    physical_location: None,
                    party_tax_scheme: vec![],
                    party_legal_entity: vec![],
                    contact: None,
                    person: None,
                    agent_party: None,
                }),
            },
            document_response: vec![cac::DocumentResponse {
                response: cac::Response {
                    reference_id: None,
                    response_code: Some(cbc::ResponseCode::new("AP")),
                    description: vec![cbc::Description::new("Invoice approved")],
                    effective_date: None,
                    effective_time: None,
                    status: vec![],
                },
                document_reference: vec![cac::DocumentReference {
                    id: Some(cbc::ID::new("INV-001")),
                    copy_indicator: None,
                    uuid: None,
                    issue_date: None,
                    issue_time: None,
                    document_type_code: None,
                    document_type: None,
                    xpath: vec![],
                    referenced_document_internal_address: None,
                    language_id: None,
                    locale_code: None,
                    version_id: None,
                    document_status_code: None,
                    document_description: vec![],
                    attachment: None,
                    validity_period: None,
                    issuer_party: None,
                    result_of_verification: None,
                }],
                issuer_party: vec![],
                recipient_party: vec![],
                line_response: vec![],
            }],
            ubl_version_id: None,
            customization_id: None,
            profile_id: None,
            profile_execution_id: None,
            uuid: None,
            issue_time: None,
            response_date: None,
            response_time: None,
            note: vec![],
            version_id: None,
            signature: vec![],
        }
    }

    fn minimal_imr_rejected() -> ApplicationResponse {
        let mut imr = minimal_imr_accepted();
        imr.document_response[0].response.response_code = Some(cbc::ResponseCode::new("RE"));
        imr.document_response[0].response.description =
            vec![cbc::Description::new("Invoice rejected — incorrect amount")];
        imr.note = vec![cbc::Note::new("Invoice INV-001 rejected due to incorrect total")];
        imr
    }

    #[test]
    fn test_valid_imr_accepted_passes_all_fatal_rules() {
        let imr = minimal_imr_accepted();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(imr));
        let failures = engine.evaluate_failures();

        // No fatal failures expected for a valid IMR
        let fatals: Vec<_> = failures
            .iter()
            .filter(|f| f.severity == Some(Severity::Fatal))
            .collect();
        assert!(
            fatals.is_empty(),
            "Expected no fatal failures but got: {:?}",
            fatals
        );
    }

    #[test]
    fn test_missing_document_reference_fails_r003() {
        let mut imr = minimal_imr_accepted();
        // Remove the document reference ID
        imr.document_response[0].document_reference[0].id = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(imr));
        let failures = engine.evaluate_failures();
        assert!(
            failures
                .iter()
                .any(|f| f.rule_id == "IMR-R003" && f.severity == Some(Severity::Fatal)),
            "Expected IMR-R003 to fail when document reference ID is missing"
        );
    }

    #[test]
    fn test_rejected_without_notes_warns_r006() {
        // Use a rejected IMR but with empty notes
        let mut imr = minimal_imr_rejected();
        imr.note = vec![]; // No notes for a rejection
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(imr));
        let failures = engine.evaluate_failures();
        assert!(
            failures
                .iter()
                .any(|f| f.rule_id == "IMR-R006" && f.severity == Some(Severity::Warning)),
            "Expected IMR-R006 warning when rejected without notes"
        );
    }
}
