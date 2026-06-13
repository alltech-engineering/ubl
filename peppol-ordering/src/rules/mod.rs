pub mod code_lists; pub mod delivery; pub mod header; pub mod lines; pub mod parties; pub mod payment;
use peppol_common::rules::RuleEngine; use std::sync::Arc; use ubl_documents::ordering::Order;
pub fn ordering_rules(order: &Order) -> RuleEngine {
    let mut engine = RuleEngine::new(); let inv = Arc::new(order.clone());
    header::add_rules(&mut engine, &inv); parties::add_rules(&mut engine, &inv);
    lines::add_rules(&mut engine, &inv); delivery::add_rules(&mut engine, &inv);
    payment::add_rules(&mut engine, &inv); code_lists::add_rules(&mut engine, &inv); engine
}
