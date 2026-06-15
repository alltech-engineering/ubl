// Peppol BIS Ordering 3.0 — Code List Validation Rules
//
// Validates that coded values belong to the correct ISO / UNCL code lists.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::ordering::Order;
use peppol_common::codes::{currency_codes, payment_means_codes};

mod ord_cl001;
mod ord_cl002;
mod ord_cl003;
mod ord_cl004;
mod ord_cl005;
mod ord_cl006;
mod ord_cl007;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<Order>) {
    engine.add_rule(ord_cl001::rule(inv));
    engine.add_rule(ord_cl002::rule(inv));
    engine.add_rule(ord_cl003::rule(inv));
    engine.add_rule(ord_cl004::rule(inv));
    engine.add_rule(ord_cl005::rule(inv));
    engine.add_rule(ord_cl006::rule(inv));
    engine.add_rule(ord_cl007::rule(inv));
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
    fn test_invalid_currency_fails() {
        let mut order = minimal_order();
        order.document_currency_code = Some(cbc::DocumentCurrencyCode::new("XXX"));
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(order));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "ORD-CL001"));
    }
}
