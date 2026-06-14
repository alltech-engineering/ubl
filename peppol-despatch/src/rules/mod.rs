// Peppol BIS Despatch Advice 3.x — Business Rules
//
// Modular rule set. Each sub-module handles a domain.
// Rules use Arc<DespatchAdvice> for zero-copy evaluation.

pub mod header;
pub mod lines;
pub mod parties;
pub mod shipment;

use peppol_common::rules::RuleEngine;
use std::sync::Arc;
use ubl_documents::despatch::DespatchAdvice;

/// Build the complete rule set for Peppol BIS Despatch Advice.
pub fn despatch_rules(despatch: &DespatchAdvice) -> RuleEngine {
    let mut engine = RuleEngine::new();
    let inv = Arc::new(despatch.clone());

    header::add_rules(&mut engine, &inv);
    parties::add_rules(&mut engine, &inv);
    lines::add_rules(&mut engine, &inv);
    shipment::add_rules(&mut engine, &inv);

    engine
}
