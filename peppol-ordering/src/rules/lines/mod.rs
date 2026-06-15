// Peppol BIS Ordering 3.0 — Order Line Business Rules
//
// Validates order line items for Purchase Orders.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

mod ord_r020;
mod ord_r020b;
mod ord_r021;
mod ord_r022;
mod ord_r023;
mod ord_r024;
mod ord_r025;
mod ord_r027;
mod ord_r028;
mod ord_r029;
mod ord_r030;
mod ord_r031;
mod ord_r032;
mod ord_r033;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Order>) {
    engine.add_rule(ord_r020::rule(inv));
    engine.add_rule(ord_r020b::rule(inv));
    engine.add_rule(ord_r021::rule(inv));
    engine.add_rule(ord_r022::rule(inv));
    engine.add_rule(ord_r023::rule(inv));
    engine.add_rule(ord_r024::rule(inv));
    engine.add_rule(ord_r025::rule(inv));
    engine.add_rule(ord_r027::rule(inv));
    engine.add_rule(ord_r028::rule(inv));
    engine.add_rule(ord_r029::rule(inv));
    engine.add_rule(ord_r030::rule(inv));
    engine.add_rule(ord_r031::rule(inv));
    engine.add_rule(ord_r032::rule(inv));
    engine.add_rule(ord_r033::rule(inv));
}

#[cfg(test)]
mod tests {
    use super::*;
    use peppol_common::rules::RuleEngine;
    use std::sync::Arc;
    use ubl_common::cbc;
    use ubl_documents::ordering::Order;

    fn minimal_order() -> Order {
        serde_json::from_str(
            r#"{
            "id": {"value": "ORD-001"},
            "issue_date": "2026-06-13",
            "document_currency_code": {"value": "ZAR"},
            "buyer_customer_party": {
                "party": {
                    "party_name": [{"name": "Buyer Ltd"}],
                    "party_identification": [{"id": {"value": "9933:buyer123"}}],
                    "postal_address": {
                        "street_name": "100 Buyer St",
                        "city_name": "Cape Town",
                        "country": {"identification_code": {"value": "ZA"}}
                    }
                }
            },
            "seller_supplier_party": {
                "party": {
                    "party_name": [{"name": "Supplier Corp"}],
                    "party_identification": [{"id": {"value": "9933:supplier456"}}],
                    "postal_address": {
                        "street_name": "200 Supplier Ave",
                        "city_name": "Johannesburg",
                        "country": {"identification_code": {"value": "ZA"}}
                    }
                }
            },
            "order_line": [{
                "note": [],
                "line_item": {
                    "id": {"value": "1"},
                    "quantity": {"value": "10", "unit_code": "EA"},
                    "line_extension_amount": {"value": "1000.00", "currency_id": "ZAR"},
                    "item": {"name": "Widget"},
                    "price": {"price_amount": {"value": "100.00", "currency_id": "ZAR"}}
                }
            }]
        }"#,
        )
        .unwrap()
    }

    #[test]
    fn test_no_lines_fails() {
        let mut order = minimal_order();
        order.order_line.clear();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R020"));
    }

    #[test]
    fn test_line_no_line_item_fails() {
        let mut order = minimal_order();
        order.order_line[0].line_item = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R020b"));
    }

    #[test]
    fn test_line_no_id_fails() {
        let mut order = minimal_order();
        if let Some(ref mut li) = order.order_line[0].line_item {
            li.id = None;
        }
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R021"));
    }

    #[test]
    fn test_line_no_item_name_fails() {
        let mut order = minimal_order();
        if let Some(ref mut li) = order.order_line[0].line_item {
            if let Some(ref mut item) = li.item {
                item.name = None;
            }
        }
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R022"));
    }

    #[test]
    fn test_line_no_quantity_fails() {
        let mut order = minimal_order();
        if let Some(ref mut li) = order.order_line[0].line_item {
            li.quantity = None;
        }
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R023"));
    }
}
