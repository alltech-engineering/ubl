// Peppol XML Serialization
//
// Wraps ubl-xml with Peppol BIS identifiers:
//   - CustomizationID and ProfileID in document root
//   - EndpointID with schemeID on party identifications
//   - Full Peppol namespace handling

use peppol_common::identity::DocumentIdentity;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::io::Cursor;
use ubl_xml::ser::ToXml;

/// A UBL document with Peppol metadata ready for serialization.
pub struct PeppolDocument<T: ToXml> {
    pub document: T,
    pub identity: DocumentIdentity,
    pub root_element: &'static str,
    pub root_namespace: &'static str,
}

/// Convert quick_xml errors to ubl_xml errors.
fn xml_err(e: quick_xml::Error) -> ubl_xml::error::Error {
    ubl_xml::error::Error::Xml(e.to_string())
}

/// Serialize a Peppol-wrapped document to XML string.
pub fn to_peppol_xml<T: ToXml>(
    peppol: &PeppolDocument<T>,
) -> Result<String, ubl_xml::error::Error> {
    // XML declaration
    let mut w = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
    w.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;

    // Root element with namespaces
    let mut root = BytesStart::new(peppol.root_element);
    root.push_attribute(("xmlns", peppol.root_namespace));
    root.push_attribute((
        "xmlns:cac",
        "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2",
    ));
    root.push_attribute((
        "xmlns:cbc",
        "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2",
    ));
    w.write_event(Event::Start(root))?;

    // Peppol identifiers as first child elements (per XSD sequence)
    write_cbc_element(&mut w, "CustomizationID", &peppol.identity.customization_id)?;
    write_cbc_element(&mut w, "ProfileID", &peppol.identity.profile_id)?;

    // Delegate the rest to the UBL serializer
    peppol.document.to_xml(&mut w)?;

    w.write_event(Event::End(BytesEnd::new(peppol.root_element)))?;
    Ok(String::from_utf8(w.into_inner().into_inner()).unwrap())
}

/// Write a simple CBC element with text content.
fn write_cbc_element(
    w: &mut Writer<Cursor<Vec<u8>>>,
    name: &str,
    value: &str,
) -> Result<(), quick_xml::Error> {
    let tag = format!("cbc:{}", name);
    w.write_event(Event::Start(BytesStart::new(&tag)))?;
    w.write_event(Event::Text(BytesText::new(value)))?;
    w.write_event(Event::End(BytesEnd::new(&tag)))?;
    Ok(())
}

/// Helper: write a CBC element with a schemeID attribute (for EndpointID, etc.)
pub fn write_cbc_with_scheme(
    w: &mut Writer<Cursor<Vec<u8>>>,
    name: &str,
    value: &str,
    scheme_id: &str,
) -> Result<(), quick_xml::Error> {
    let tag = format!("cbc:{}", name);
    let mut elem = BytesStart::new(&tag);
    elem.push_attribute(("schemeID", scheme_id));
    w.write_event(Event::Start(elem))?;
    w.write_event(Event::Text(BytesText::new(value)))?;
    w.write_event(Event::End(BytesEnd::new(&tag)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ubl_documents::billing::Invoice;
    use peppol_common::identity::identities;

    #[test]
    fn test_peppol_invoice_xml_contains_identifiers() {
        let json = r#"{
            "id": {"value": "INV-001"},
            "issue_date": "2026-06-13",
            "invoice_type_code": {"value": "380"},
            "document_currency_code": {"value": "ZAR"},
            "note": [{"value": "Test invoice"}],
            "accounting_supplier_party": {
                "party": {
                    "party_name": [{"name": "Acme Corp"}],
                    "party_identification": [{"id": {"value": "9933:za1234567890"}}],
                    "postal_address": {
                        "street_name": "123 Main St",
                        "city_name": "Cape Town",
                        "postal_zone": {"value": "8001"},
                        "country": {"identification_code": {"value": "ZA"}},
                        "address_line": []
                    }
                }
            },
            "legal_monetary_total": {
                "line_extension_amount": {"value": "100.00", "currency_id": "ZAR"},
                "payable_amount": {"value": "115.00", "currency_id": "ZAR"}
            },
            "invoice_line": [{
                "id": {"value": "1"},
                "line_extension_amount": {"value": "100.00", "currency_id": "ZAR"},
                "item": {"name": "Widget"}
            }]
        }"#;

        let invoice: Invoice = serde_json::from_str(json).unwrap();
        let identity = identities::billing_3_0("Invoice");

        let peppol = PeppolDocument {
            document: invoice,
            identity,
            root_element: "Invoice",
            root_namespace: "urn:oasis:names:specification:ubl:schema:xsd:Invoice-2",
        };

        let xml = to_peppol_xml(&peppol).unwrap();

        assert!(xml.contains("CustomizationID"), "Missing CustomizationID");
        assert!(xml.contains("ProfileID"), "Missing ProfileID");
        assert!(xml.contains("billing:3.0"), "Missing billing URN");
        assert!(xml.contains("billing:01"), "Missing profile URN");
        assert!(xml.contains("<cbc:ID>INV-001</cbc:ID>"), "Missing invoice ID");
    }
}
