use ubl_documents::billing::Invoice;
use ubl_xml::ser::to_string;

fn main() {
    let json = r#"{
        "id": {"value": "INV-001"},
        "issue_date": "2026-06-12",
        "invoice_type_code": {"value": "380"},
        "document_currency_code": {"value": "ZAR"},
        "note": [{"value": "Thank you for your business"}],
        "accounting_supplier_party": {
            "party": {
                "party_name": [{"name": "Acme Corp"}],
                "postal_address": {
                    "street_name": "123 Main St",
                    "city_name": "Cape Town",
                    "postal_zone": {"value": "8001"},
                    "country": {"identification_code": {"value": "ZA"}},
                    "address_line": []
                },
                "party_tax_scheme": [{
                    "registration_name": "Acme Corp",
                    "company_id": {"value": "9876543210"},
                    "tax_scheme": {"id": {"value": "VAT"}, "name": "VAT"}
                }]
            }
        },
        "legal_monetary_total": {
            "line_extension_amount": {"value": "100.00", "currency_id": "ZAR"},
            "tax_exclusive_amount": {"value": "100.00", "currency_id": "ZAR"},
            "payable_amount": {"value": "115.00", "currency_id": "ZAR"}
        },
        "tax_total": [{
            "tax_amount": {"value": "15.00", "currency_id": "ZAR"},
            "tax_subtotal": [{
                "tax_amount": {"value": "15.00", "currency_id": "ZAR"},
                "taxable_amount": {"value": "100.00", "currency_id": "ZAR"},
                "percent": "15",
                "tax_category": {
                    "id": {"value": "S"},
                    "name": "Standard Rate",
                    "percent": "15",
                    "tax_scheme": {"id": {"value": "VAT"}, "name": "VAT"}
                }
            }]
        }],
        "invoice_line": [{
            "id": {"value": "1"},
            "invoiced_quantity": {"value": "5"},
            "line_extension_amount": {"value": "100.00", "currency_id": "ZAR"},
            "item": {"description": {"value": "Widget, Model X"}, "name": "Widget"}
        }]
    }"#;
    let invoice: Invoice = serde_json::from_str(json).unwrap();
    let xml = to_string(&invoice, "Invoice").unwrap();
    println!("{}", xml);
}
