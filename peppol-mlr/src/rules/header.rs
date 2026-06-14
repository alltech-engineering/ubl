// Peppol BIS MLR 3.0 — Header & Document Rules
//
// Validates the ApplicationResponse (Message Level Response) per Peppol BIS MLR 3.0.
// This is the simplest Peppol BIS — only ~8 rules.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::status::ApplicationResponse;

pub fn add_rules(engine: &mut RuleEngine, mlr: &Arc<ApplicationResponse>) {
    // ── MLR-R001: MLR ID must be present (Fatal) ─────────────────────────
    engine.add_rule(Rule {
        id: "MLR-R001".into(),
        description: "MLR ID must be present and non-empty".into(),
        severity: Severity::Fatal,
        check: {
            let mlr = Arc::clone(mlr);
            Box::new(move || {
                let v = mlr.id.value();
                if v.is_empty() {
                    Err("MLR ID is empty — a non-empty identifier is required".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── MLR-R002: Issue date must be present (Fatal) ─────────────────────
    engine.add_rule(Rule {
        id: "MLR-R002".into(),
        description: "MLR issue date must be present".into(),
        severity: Severity::Fatal,
        check: {
            let mlr = Arc::clone(mlr);
            Box::new(move || {
                // issue_date is a required struct field — always present.
                // This rule passes trivially but is included for completeness.
                let _date = &mlr.issue_date;
                Ok(())
            })
        },
    });

    // ── MLR-R003: Must reference the document being acknowledged (Fatal) ─
    engine.add_rule(Rule {
        id: "MLR-R003".into(),
        description: "Must reference the document being acknowledged (DocumentResponse.DocumentReference.ID)".into(),
        severity: Severity::Fatal,
        check: {
            let mlr = Arc::clone(mlr);
            Box::new(move || {
                if mlr.document_response.is_empty() {
                    return Err(
                        "No DocumentResponse present — must reference the document being acknowledged"
                            .into(),
                    );
                }
                let has_ref = mlr.document_response.iter().any(|dr| {
                    dr.document_reference.iter().any(|dref| {
                        dref.id.as_ref().map(|id| !id.value().is_empty()).unwrap_or(false)
                    })
                });
                if has_ref {
                    Ok(())
                } else {
                    Err(
                        "No DocumentReference with a non-empty ID found — must reference the document being acknowledged"
                            .into(),
                    )
                }
            })
        },
    });

    // ── MLR-R004: Response code must be present (Fatal) ──────────────────
    engine.add_rule(Rule {
        id: "MLR-R004".into(),
        description: "Response code must be present (Response.ResponseCode)".into(),
        severity: Severity::Fatal,
        check: {
            let mlr = Arc::clone(mlr);
            Box::new(move || {
                if mlr.document_response.is_empty() {
                    return Err(
                        "No DocumentResponse present — response code is required".into(),
                    );
                }
                let has_code = mlr.document_response.iter().any(|dr| {
                    dr.response.response_code.is_some()
                });
                if has_code {
                    Ok(())
                } else {
                    Err(
                        "Response.ResponseCode is missing — a response code (e.g., CA, RE) is required"
                            .into(),
                    )
                }
            })
        },
    });

    // ── MLR-R005: Response description should explain the outcome (Warning) ─
    engine.add_rule(Rule {
        id: "MLR-R005".into(),
        description: "Response description should explain the outcome".into(),
        severity: Severity::Warning,
        check: {
            let mlr = Arc::clone(mlr);
            Box::new(move || {
                if mlr.document_response.is_empty() {
                    return Err(
                        "No DocumentResponse with a description — provide context about the response outcome"
                            .into(),
                    );
                }
                let has_desc = mlr.document_response.iter().any(|dr| {
                    !dr.response.description.is_empty()
                        && dr.response.description.iter().any(|d| !d.value().trim().is_empty())
                });
                if has_desc {
                    Ok(())
                } else {
                    Err(
                        "Response description is missing or empty — should explain the outcome"
                            .into(),
                    )
                }
            })
        },
    });

    // ── MLR-R006: Sender party must be present (Fatal) ───────────────────
    engine.add_rule(Rule {
        id: "MLR-R006".into(),
        description: "Sender party must be present".into(),
        severity: Severity::Fatal,
        check: {
            let mlr = Arc::clone(mlr);
            Box::new(move || {
                if mlr.sender_party.party.is_some() {
                    Ok(())
                } else {
                    Err("Sender party is missing — the MLR must identify who sent it".into())
                }
            })
        },
    });

    // ── MLR-R007: Receiver party must be present (Fatal) ─────────────────
    engine.add_rule(Rule {
        id: "MLR-R007".into(),
        description: "Receiver party must be present".into(),
        severity: Severity::Fatal,
        check: {
            let mlr = Arc::clone(mlr);
            Box::new(move || {
                if mlr.receiver_party.party.is_some() {
                    Ok(())
                } else {
                    Err("Receiver party is missing — the MLR must identify the recipient".into())
                }
            })
        },
    });

    // ── MLR-R008: Notes should provide context (Warning) ─────────────────
    engine.add_rule(Rule {
        id: "MLR-R008".into(),
        description: "Notes should provide context about the response".into(),
        severity: Severity::Warning,
        check: {
            let mlr = Arc::clone(mlr);
            Box::new(move || {
                if mlr.note.is_empty() {
                    Err("No notes provided — consider adding context about the response".into())
                } else if mlr.note.iter().all(|n| n.value().trim().is_empty()) {
                    Err(
                        "Notes are present but all are empty — provide meaningful context".into(),
                    )
                } else {
                    Ok(())
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

    fn minimal_mlr() -> ApplicationResponse {
        use chrono::NaiveDate;
        ApplicationResponse {
            id: cbc::ID::new("MLR-001"),
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
            document_response: vec![cac::DocumentResponse {
                response: cac::Response {
                    reference_id: None,
                    response_code: Some(cbc::ResponseCode::new("CA")),
                    description: vec![cbc::Description::new("Document accepted")],
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
            note: vec![cbc::Note::new("Invoice INV-001 accepted")],
            version_id: None,
            signature: vec![],
        }
    }

    #[test]
    fn test_valid_mlr_passes_all_rules() {
        let mlr = minimal_mlr();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(mlr));
        let failures = engine.evaluate_failures();
        // No fatal failures expected for a valid MLR
        let fatals: Vec<_> = failures
            .iter()
            .filter(|f| f.severity == Some(Severity::Fatal))
            .collect();
        assert!(
            fatals.is_empty(),
            "Expected no fatal failures but got: {:?}",
            fatals
        );
        // Warnings are informational — MLR-R005 and MLR-R008 pass since we have
        // description and notes set in the minimal MLR
    }

    #[test]
    fn test_missing_doc_ref_fails_r003() {
        let mut mlr = minimal_mlr();
        // Remove the document reference ID
        mlr.document_response[0].document_reference[0].id = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(mlr));
        let failures = engine.evaluate_failures();
        assert!(
            failures.iter().any(|f| f.rule_id == "MLR-R003"
                && f.severity == Some(Severity::Fatal)),
            "Expected MLR-R003 to fail when document reference ID is missing"
        );
    }

    #[test]
    fn test_missing_response_code_fails_r004() {
        let mut mlr = minimal_mlr();
        // Remove the response code
        mlr.document_response[0].response.response_code = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(mlr));
        let failures = engine.evaluate_failures();
        assert!(
            failures.iter().any(|f| f.rule_id == "MLR-R004"
                && f.severity == Some(Severity::Fatal)),
            "Expected MLR-R004 to fail when response code is missing"
        );
    }
}
