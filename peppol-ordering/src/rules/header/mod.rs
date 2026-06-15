// Peppol BIS Ordering 3.0 — Document Header Rules
//
// Validates document-level metadata for Purchase Orders.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

mod ord_r001;
mod ord_r002;
mod ord_r003;
mod ord_r004;
mod ord_r005;
mod ord_r006;
mod ord_r007;
mod ord_r008;
mod ord_r009;
mod ord_r010;
mod ord_r011;
mod ord_r012;
mod ord_r013;
mod ord_r014;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Order>) {
    engine.add_rule(ord_r001::rule(inv));
    engine.add_rule(ord_r002::rule(inv));
    engine.add_rule(ord_r003::rule(inv));
    engine.add_rule(ord_r004::rule(inv));
    engine.add_rule(ord_r005::rule(inv));
    engine.add_rule(ord_r006::rule(inv));
    engine.add_rule(ord_r007::rule(inv));
    engine.add_rule(ord_r008::rule(inv));
    engine.add_rule(ord_r009::rule(inv));
    engine.add_rule(ord_r010::rule(inv));
    engine.add_rule(ord_r011::rule(inv));
    engine.add_rule(ord_r012::rule(inv));
    engine.add_rule(ord_r013::rule(inv));
    engine.add_rule(ord_r014::rule(inv));
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
            "validity_period": [{}],
            "note": [{"value": "Test order"}],
            "quotation_document_reference": {},
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
                "id": {"value": "1"},
                "quantity": {"value": "10", "unit_code": "EA"},
                "line_extension_amount": {"value": "1000.00", "currency_id": "ZAR"},
                "item": {"name": "Widget"},
                "price": {"price_amount": {"value": "100.00", "currency_id": "ZAR"}}
            }]
        }"#,
        )
        .unwrap()
    }

    #[test]
    fn test_valid_order_passes_all_header_rules() {
        let order = minimal_order();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(
            failures.is_empty(),
            "Expected no failures but got: {:?}",
            failures
        );
    }

    #[test]
    fn test_missing_order_id_fails() {
        let mut order = minimal_order();
        order.id = cbc::ID::new("");
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R001"));
    }

    #[test]
    fn test_missing_currency_fails() {
        let mut order = minimal_order();
        order.document_currency_code = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R003"));
    }

    #[test]
    fn test_missing_buyer_reference_warns() {
        let mut order = minimal_order();
        order.quotation_document_reference = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R006"));
    }
}
