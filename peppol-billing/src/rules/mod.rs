// Peppol BIS Billing 3.0 Business Rules
//
// Modular rule set. Each sub-module handles a domain.
// Rules use Arc<Invoice> for zero-copy evaluation.

pub mod code_lists;
pub mod header;
pub mod lines;
pub mod parties;
pub mod tax_calc;
pub mod za;

use peppol_common::rules::RuleEngine;
use std::sync::Arc;
use ubl_documents::billing::Invoice;

/// Build the complete rule set for Peppol BIS Billing 3.0.
pub fn billing_rules(invoice: &Invoice) -> RuleEngine {
    let mut engine = RuleEngine::new();
    let inv = Arc::new(invoice.clone());

    header::add_rules(&mut engine, &inv);
    parties::add_rules(&mut engine, &inv);
    lines::add_rules(&mut engine, &inv);
    tax_calc::add_rules(&mut engine, &inv);
    code_lists::add_rules(&mut engine, &inv);
    za::add_rules(&mut engine, &inv);

    engine
}
