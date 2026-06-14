pub mod header;
pub mod lines;

use peppol_common::rules::RuleEngine;
use std::sync::Arc;
use ubl_documents::catalogue::Catalogue;

pub fn catalogue_rules(catalogue: &Catalogue) -> RuleEngine {
    let mut engine = RuleEngine::new();
    let cat = Arc::new(catalogue.clone());

    header::add_rules(&mut engine, &cat);
    lines::add_rules(&mut engine, &cat);

    engine
}
