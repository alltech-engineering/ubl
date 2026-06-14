// Peppol BIS Invoice Message Response 3.0
//
// Implements the Peppol BIS IMR 3.0 specification on top of UBL.
// The IMR is a business-level response to an invoice — "I approve" or "I reject."
// Uses the same UBL ApplicationResponse struct as MLR but with different business rules.
//
// Reference: https://docs.peppol.eu/poacc/imr/3.0/

pub mod rules;

use peppol_common::identity::identities::imr_3_0;
use peppol_common::identity::{BisDocument, DocumentIdentity};
use peppol_common::rules::RuleEngine;
use ubl_documents::status::ApplicationResponse;

/// A Peppol BIS Invoice Message Response 3.0 wrapper.
///
/// Carries the UBL ApplicationResponse plus Peppol-specific metadata and validation.
pub struct PeppolImr {
    /// The underlying UBL ApplicationResponse
    pub response: ApplicationResponse,
    /// The document identity declaring BIS compliance
    identity: DocumentIdentity,
    /// The Peppol validation rule engine
    #[allow(dead_code)]
    engine: RuleEngine,
}

impl PeppolImr {
    /// Create a new Peppol IMR with the IMR 3.0 identity.
    pub fn new(response: ApplicationResponse) -> Self {
        Self {
            response,
            identity: imr_3_0(),
            engine: RuleEngine::new(),
        }
    }

    /// Validate this IMR against all Peppol IMR 3.0 rules.
    pub fn validate(&self) -> Vec<peppol_common::rules::RuleOutcome> {
        let engine = rules::imr_rules(&self.response);
        engine.evaluate_all()
    }
}

impl BisDocument for PeppolImr {
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
    fn test_peppol_imr_identity() {
        let json = r#"{
            "id": {"value": "IMR-001"},
            "issue_date": "2026-06-12",
            "sender_party": {
                "party": {
                    "party_name": [{"name": "Beta Ltd"}]
                }
            },
            "receiver_party": {
                "party": {
                    "party_name": [{"name": "Acme Corp"}]
                }
            },
            "document_response": [{
                "response": {
                    "response_code": {"value": "RE"},
                    "description": [{"value": "Invoice rejected — incorrect amount"}]
                },
                "document_reference": [{
                    "id": {"value": "INV-001"}
                }]
            }],
            "note": [{"value": "Invoice INV-001 rejected due to incorrect total"}]
        }"#;
        let imr: ApplicationResponse = serde_json::from_str(json).unwrap();
        let peppol = PeppolImr::new(imr);
        assert!(peppol.identity().customization_id.contains("imr:3.0"));
        assert!(peppol.identity().profile_id.contains("imr:01"));
    }
}
