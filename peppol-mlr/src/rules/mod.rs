// Peppol BIS Message Level Response 3.0 Business Rules
//
// Lightweight rule set — MLR is the simplest Peppol document.
// Rules use Arc<ApplicationResponse> for zero-copy evaluation.

pub mod header;

use peppol_common::rules::RuleEngine;
use std::sync::Arc;
use ubl_documents::status::ApplicationResponse;

/// Build the complete rule set for Peppol BIS MLR 3.0.
pub fn mlr_rules(response: &ApplicationResponse) -> RuleEngine {
    let mut engine = RuleEngine::new();
    let inv = Arc::new(response.clone());

    header::add_rules(&mut engine, &inv);

    engine
}
