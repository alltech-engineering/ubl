pub mod code_lists;
pub mod delivery;
pub mod header;
pub mod lines;
pub mod order_response;
pub mod parties;
pub mod payment;
use peppol_common::rules::RuleEngine;
use std::sync::Arc;
use ubl_documents::ordering::Order;
use ubl_documents::ordering::OrderResponse;
pub fn ordering_rules(order: &Order) -> RuleEngine {
    let mut engine = RuleEngine::new();
    let inv = Arc::new(order.clone());
    header::add_rules(&mut engine, &inv);
    parties::add_rules(&mut engine, &inv);
    lines::add_rules(&mut engine, &inv);
    delivery::add_rules(&mut engine, &inv);
    payment::add_rules(&mut engine, &inv);
    code_lists::add_rules(&mut engine, &inv);
    engine
}
pub fn ordering_response_rules(response: &OrderResponse) -> RuleEngine {
    let mut engine = RuleEngine::new();
    let inv = Arc::new(response.clone());
    order_response::add_rules(&mut engine, &inv);
    engine
}
