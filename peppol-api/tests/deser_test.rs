// Integration test: deserialize full Order JSON from the web form
// Run with: cargo test -p peppol-api --test deser_test

use serde::Deserialize;
use ubl_documents::ordering::Order;

#[test]
fn test_deserialize_dummy_order_json() {
    let json = r#"{
        "id": {"value": "ORD-2026-001"},
        "issue_date": "2026-06-15",
        "order_type_code": {"value": "220"},
        "ubl_version_id": {"value": "2.5"},
        "document_currency_code": {"value": "ZAR"},
        "customer_reference": {"value": "PO-REF-001"},
        "note": [{"value": "Test order"}],
        "quotation_document_reference": {
            "id": {"value": "QUOT-2026-042"}
        },
        "buyer_customer_party": {
            "party": {
                "party_name": [{"name": "Acme Buyer Pty Ltd"}],
                "party_identification": [{"id": {"value": "4123456789", "scheme_id": "9933"}}],
                "party_legal_entity": [{"registration_name": "Acme Buyer Pty Ltd", "company_id": {"value": "2022/123456/07"}}],
                "party_tax_scheme": [{"company_id": {"value": "4123456789"}, "tax_scheme": {"id": {"value": "VAT"}}}],
                "postal_address": {
                    "street_name": "100 Main Street",
                    "city_name": "Cape Town",
                    "postal_zone": {"value": "8001"},
                    "country": {"identification_code": {"value": "ZA"}},
                    "address_line": []
                },
                "endpoint_id": {"value": "9933:4123456789"},
                "contact": {
                    "name": "John Buyer",
                    "telephone": {"value": "021-555-1234"},
                    "electronic_mail": {"value": "john@acme.co.za"}
                }
            }
        },
        "seller_supplier_party": {
            "party": {
                "party_name": [{"name": "ZA Supplies Pty Ltd"}],
                "party_identification": [{"id": {"value": "4987654321", "scheme_id": "9933"}}],
                "party_legal_entity": [{"registration_name": "ZA Supplies Pty Ltd", "company_id": {"value": "2020/654321/07"}}],
                "party_tax_scheme": [{"company_id": {"value": "4987654321"}, "tax_scheme": {"id": {"value": "VAT"}}}],
                "postal_address": {
                    "street_name": "200 Industrial Road",
                    "city_name": "Johannesburg",
                    "postal_zone": {"value": "2000"},
                    "country": {"identification_code": {"value": "ZA"}},
                    "address_line": []
                },
                "endpoint_id": {"value": "9933:4987654321"},
                "contact": {
                    "name": "Jane Seller",
                    "telephone": {"value": "011-555-9876"},
                    "electronic_mail": {"value": "jane@zasupplies.co.za"}
                }
            }
        },
        "payment_means": [{"payment_means_code": {"value": "30"}}],
        "payment_terms": [{"note": [{"value": "Net 30 days from invoice date"}]}],
        "tax_total": [{
            "tax_amount": {"value": "0", "currency_id": "ZAR"},
            "tax_subtotal": [{
                "taxable_amount": {"value": "0", "currency_id": "ZAR"},
                "tax_amount": {"value": "0", "currency_id": "ZAR"},
                "tax_category": {
                    "id": {"value": "S"},
                    "tax_scheme": {"id": {"value": "VAT"}}
                }
            }]
        }],
        "order_line": [{
            "line_item": {
                "id": {"value": "1"},
                "quantity": {"value": "10", "unit_code": "EA"},
                "line_extension_amount": {"value": "1599.50", "currency_id": "ZAR"},
                "item": {
                    "name": "HP EliteBook 840 G10",
                    "description": {"value": "14 inch business laptop"},
                    "sellers_item_identification": {"id": {"value": "HP-840-G10"}},
                    "standard_item_identification": {"id": {"value": "0196337175246", "scheme_id": "GTIN"}},
                    "commodity_classification": [{"item_classification_code": {"value": "84713000"}}]
                },
                "price": {
                    "price_amount": {"value": "159.95", "currency_id": "ZAR"},
                    "base_quantity": {"value": "1", "unit_code": "EA"}
                }
            }
        }]
    }"#;

    match serde_json::from_str::<Order>(json) {
        Ok(order) => {
            println!("DESERIALIZE SUCCESS");
            println!("Order ID: {:?}", order.id);
            if let Some(ref party) = order.buyer_customer_party.party {
                println!("Buyer: {:?}", party.party_name.first().map(|n| &n.name));
            }
            println!("Lines: {}", order.order_line.len());
        }
        Err(e) => {
            panic!("DESERIALIZE FAILED:\n{}", e);
        }
    }
}
