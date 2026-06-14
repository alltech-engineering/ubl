// Peppol BIS Invoice Message Response 3.0 Business Rules
//
// IMR is the business-level response to an invoice — "I approve" or "I reject."
// Uses the same ApplicationResponse struct as MLR but with different business rules.
// Rules use Arc<ApplicationResponse> for zero-copy evaluation.

pub mod header;

use peppol_common::rules::RuleEngine;
use std::sync::Arc;
use ubl_documents::status::ApplicationResponse;

/// Build the complete rule set for Peppol BIS IMR 3.0.
pub fn imr_rules(response: &ApplicationResponse) -> RuleEngine {
    let mut engine = RuleEngine::new();
    let inv = Arc::new(response.clone());

    header::add_rules(&mut engine, &inv);

    engine
}
