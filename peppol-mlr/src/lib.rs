// Peppol BIS Message Level Response 3.0
//
// Implements the Peppol BIS MLR 3.0 specification on top of UBL.
// The MLR is the simplest Peppol document — just an acknowledgment.
//
// Reference: https://docs.peppol.eu/poacc/mlr/3.0/

pub mod rules;

use peppol_common::identity::identities::mlr_3_0;
use peppol_common::identity::{BisDocument, DocumentIdentity};
use peppol_common::rules::RuleEngine;
use ubl_documents::status::ApplicationResponse;

/// A Peppol BIS Message Level Response 3.0 wrapper.
///
/// Carries the UBL ApplicationResponse plus Peppol-specific metadata and validation.
pub struct PeppolMlr {
    /// The underlying UBL ApplicationResponse
    pub response: ApplicationResponse,
    /// The document identity declaring BIS compliance
    identity: DocumentIdentity,
    /// The Peppol validation rule engine
    #[allow(dead_code)]
    engine: RuleEngine,
}

impl PeppolMlr {
    /// Create a new Peppol MLR with the MLR 3.0 identity.
    pub fn new(response: ApplicationResponse) -> Self {
        Self {
            response,
            identity: mlr_3_0(),
            engine: RuleEngine::new(),
        }
    }

    /// Validate this MLR against all Peppol MLR 3.0 rules.
    pub fn validate(&self) -> Vec<peppol_common::rules::RuleOutcome> {
        let engine = rules::mlr_rules(&self.response);
        engine.evaluate_all()
    }
}

impl BisDocument for PeppolMlr {
    fn document_type() -> &'static str {
        "ApplicationResponse"
    }

    fn identity(&self) -> &DocumentIdentity {
        &self.identity
    }

    fn validate_peppol(&self) -> Vec<peppol_common::rules::RuleOutcome> {
        self.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peppol_mlr_identity() {
        let json = r#"{
            "id": {"value": "MLR-001"},
            "issue_date": "2026-06-12",
            "sender_party": {
                "party": {
                    "party_name": [{"name": "Acme Corp"}]
                }
            },
            "receiver_party": {
                "party": {
                    "party_name": [{"name": "Beta Ltd"}]
                }
            },
            "document_response": [{
                "response": {
                    "response_code": {"value": "CA"}
                },
                "document_reference": [{
                    "id": {"value": "INV-001"}
                }]
            }]
        }"#;
        let mlr: ApplicationResponse = serde_json::from_str(json).unwrap();
        let peppol = PeppolMlr::new(mlr);
        assert!(peppol.identity().customization_id.contains("mlr:3.0"));
    }
}
