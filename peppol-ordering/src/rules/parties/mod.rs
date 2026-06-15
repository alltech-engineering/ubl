// Peppol BIS Ordering 3.0 — Party Business Rules
//
// Validates buyer (BuyerCustomerParty) and seller (SellerSupplierParty)
// party information for Purchase Orders.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;

mod ord_r010;
mod ord_r011;
mod ord_r012;
mod ord_r013;
mod ord_r014;
mod ord_r015;
mod ord_r016;
mod ord_r017;
mod ord_r018;
mod ord_r019;
mod ord_r020;
mod ord_r021;
mod ord_r022;
mod ord_r023;
mod ord_r024;
mod ord_r025;
mod ord_r026;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Order>) {
    engine.add_rule(ord_r010::rule(inv));
    engine.add_rule(ord_r011::rule(inv));
    engine.add_rule(ord_r012::rule(inv));
    engine.add_rule(ord_r013::rule(inv));
    engine.add_rule(ord_r014::rule(inv));
    engine.add_rule(ord_r015::rule(inv));
    engine.add_rule(ord_r016::rule(inv));
    engine.add_rule(ord_r017::rule(inv));
    engine.add_rule(ord_r018::rule(inv));
    engine.add_rule(ord_r019::rule(inv));
    engine.add_rule(ord_r020::rule(inv));
    engine.add_rule(ord_r021::rule(inv));
    engine.add_rule(ord_r022::rule(inv));
    engine.add_rule(ord_r023::rule(inv));
    engine.add_rule(ord_r024::rule(inv));
    engine.add_rule(ord_r025::rule(inv));
    engine.add_rule(ord_r026::rule(inv));
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
    fn test_missing_buyer_party_fails() {
        let mut order = minimal_order();
        order.buyer_customer_party.party = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R010"));
    }

    #[test]
    fn test_missing_seller_party_fails() {
        let mut order = minimal_order();
        order.seller_supplier_party.party = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R012"));
    }

    #[test]
    fn test_missing_buyer_id_fails() {
        let mut order = minimal_order();
        order
            .buyer_customer_party
            .party
            .as_mut()
            .unwrap()
            .party_identification
            .clear();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R011"));
    }

    #[test]
    fn test_missing_seller_id_fails() {
        let mut order = minimal_order();
        order
            .seller_supplier_party
            .party
            .as_mut()
            .unwrap()
            .party_identification
            .clear();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R013"));
    }

    #[test]
    fn test_missing_country_warns() {
        let mut order = minimal_order();
        order
            .buyer_customer_party
            .party
            .as_mut()
            .unwrap()
            .postal_address
            .as_mut()
            .unwrap()
            .country = None;
        order
            .seller_supplier_party
            .party
            .as_mut()
            .unwrap()
            .postal_address
            .as_mut()
            .unwrap()
            .country = None;
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R014"));
        assert!(failures.iter().any(|f| f.rule_id == "ORD-R015"));
    }
}
