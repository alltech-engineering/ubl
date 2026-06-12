// UBL XML Serializer for Invoice documents.
// Uses explicit field access — no Display impl needed on domain types.

use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::io::Cursor;
use crate::error::{Error, Result};
use crate::ns::to_element_name;
use ubl_common::cbc;
use ubl_common::cac;
use ubl_documents::billing::Invoice;

pub fn to_string<T: ToXml>(doc: &T, doc_type: &str) -> Result<String> {
    let mut w = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
    w.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
    let mut root = BytesStart::new(doc_type);
    root.push_attribute(("xmlns","urn:oasis:names:specification:ubl:schema:xsd:Invoice-2"));
    root.push_attribute(("xmlns:cac","urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"));
    root.push_attribute(("xmlns:cbc","urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2"));
    w.write_event(Event::Start(root))?;
    doc.to_xml(&mut w)?;
    w.write_event(Event::End(BytesEnd::new(doc_type)))?;
    Ok(String::from_utf8(w.into_inner().into_inner()).map_err(|e| Error::Xml(e.to_string()))?)
}

pub trait ToXml { fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()>; }

fn el(w: &mut Writer<Cursor<Vec<u8>>>, name: &str, val: &str) -> Result<()> {
    let t = format!("cbc:{}", to_element_name(name));
    w.write_event(Event::Start(BytesStart::new(t.as_str())))?;
    w.write_event(Event::Text(BytesText::new(val)))?;
    w.write_event(Event::End(BytesEnd::new(t.as_str())))?;
    Ok(())
}
fn el_opt(w: &mut Writer<Cursor<Vec<u8>>>, name: &str, val: Option<&str>) -> Result<()> {
    if let Some(v) = val { el(w, name, v)?; } Ok(())
}
fn open(w: &mut Writer<Cursor<Vec<u8>>>, name: &str) -> Result<()> {
    w.write_event(Event::Start(BytesStart::new(format!("cac:{}",to_element_name(name)).as_str())))?; Ok(())
}
fn close(w: &mut Writer<Cursor<Vec<u8>>>, name: &str) -> Result<()> {
    w.write_event(Event::End(BytesEnd::new(format!("cac:{}",to_element_name(name)).as_str())))?; Ok(())
}
fn cac_opt<T: ToXml>(w: &mut Writer<Cursor<Vec<u8>>>, name: &str, val: Option<&T>) -> Result<()> {
    if let Some(v) = val { open(w, name)?; v.to_xml(w)?; close(w, name)?; } Ok(())
}
fn cac_vec<T: ToXml>(w: &mut Writer<Cursor<Vec<u8>>>, name: &str, vals: &[T]) -> Result<()> {
    for v in vals { open(w, name)?; v.to_xml(w)?; close(w, name)?; } Ok(())
}

// ── CAC ToXml impls ──

impl ToXml for cac::Party {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        for pi in &self.party_identification { open(w,"PartyIdentification")?; el(w,"id",pi.id.value())?; close(w,"PartyIdentification")?; }
        for pn in &self.party_name { open(w,"PartyName")?; el(w,"name",&pn.name.0)?; close(w,"PartyName")?; }
        cac_opt(w,"PostalAddress",self.postal_address.as_ref())?;
        cac_opt(w,"Contact",self.contact.as_ref())?;
        for pts in &self.party_tax_scheme {
            open(w,"PartyTaxScheme")?;
            el_opt(w,"registration_name",pts.registration_name.as_ref().map(|n| n.0.as_str()))?;
            el_opt(w,"company_id",pts.company_id.as_ref().map(|id| id.value()))?;
            open(w,"TaxScheme")?;
            el_opt(w,"id",pts.tax_scheme.id.as_ref().map(|id| id.value()))?;
            el_opt(w,"name",pts.tax_scheme.name.as_ref().map(|n| n.0.as_str()))?;
            close(w,"TaxScheme")?; close(w,"PartyTaxScheme")?;
        }
        Ok(())
    }
}

impl ToXml for cac::PostalAddress {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_opt(w,"street_name",self.street_name.as_ref().map(|n| n.0.as_str()))?;
        el_opt(w,"city_name",self.city_name.as_ref().map(|n| n.0.as_str()))?;
        el_opt(w,"postal_zone",self.postal_zone.as_ref().map(|n| n.0.value.as_str()))?;
        if let Some(ref c) = self.country {
            open(w,"Country")?;
            el_opt(w,"identification_code",c.identification_code.as_ref().map(|code| code.value()))?;
            close(w,"Country")?;
        }
        Ok(())
    }
}

impl ToXml for cac::Contact {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_opt(w,"name",self.name.as_ref().map(|n| n.0.as_str()))?;
        el_opt(w,"telephone",self.telephone.as_ref().map(|n| n.0.value.as_str()))?;
        el_opt(w,"electronic_mail",self.electronic_mail.as_ref().map(|t| t.value.as_str()))?;
        Ok(())
    }
}

impl ToXml for cac::SupplierParty { fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> { cac_opt(w,"Party",self.party.as_ref()) } }
impl ToXml for cac::CustomerParty { fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> { cac_opt(w,"Party",self.party.as_ref()) } }

impl ToXml for cac::TaxScheme {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_opt(w,"id",self.id.as_ref().map(|id| id.value()))?;
        el_opt(w,"name",self.name.as_ref().map(|n| n.0.as_str()))?;
        Ok(())
    }
}

impl ToXml for cac::TaxCategory {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_opt(w,"id",self.id.as_ref().map(|id| id.value()))?;
        if let Some(ref p) = self.percent { el(w,"percent",&p.0.to_string())?; }
        open(w,"TaxScheme")?; self.tax_scheme.to_xml(w)?; close(w,"TaxScheme")?;
        Ok(())
    }
}

impl ToXml for cac::TaxSubtotal {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el(w,"tax_amount",&format!("{:.2}",self.tax_amount.value()))?;
        if let Some(ref ta) = self.taxable_amount { el(w,"taxable_amount",&format!("{:.2}",ta.value()))?; }
        if let Some(ref p) = self.percent { el(w,"percent",&p.0.to_string())?; }
        open(w,"TaxCategory")?; self.tax_category.to_xml(w)?; close(w,"TaxCategory")?;
        Ok(())
    }
}

impl ToXml for cac::TaxTotal {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el(w,"tax_amount",&format!("{:.2}",self.tax_amount.value()))?;
        cac_vec(w,"TaxSubtotal",&self.tax_subtotal)?;
        Ok(())
    }
}

impl ToXml for cac::LegalTotal {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el(w,"line_extension_amount",&format!("{:.2}",self.line_extension_amount.value()))?;
        let te = format!("{:.2}",self.tax_exclusive_amount.as_ref().map(|a|*a.value()).unwrap_or(*self.line_extension_amount.value()));
        el(w,"tax_exclusive_amount",&te)?;
        el(w,"payable_amount",&format!("{:.2}",self.payable_amount.value()))?;
        Ok(())
    }
}

impl ToXml for cac::Item {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_opt(w,"description",self.description.as_ref().map(|d| d.value()))?;
        el_opt(w,"name",self.name.as_ref().map(|n| n.0.as_str()))?;
        Ok(())
    }
}

impl ToXml for cac::Period {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        if let Some(ref sd) = self.start_date { el(w,"start_date",&sd.0.format("%Y-%m-%d").to_string())?; }
        if let Some(ref ed) = self.end_date { el(w,"end_date",&ed.0.format("%Y-%m-%d").to_string())?; }
        Ok(())
    }
}

// ── Document ──

impl ToXml for Invoice {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el(w,"id",self.id.value())?;
        el(w,"issue_date",&self.issue_date.0.format("%Y-%m-%d").to_string())?;
        if let Some(ref n) = self.note.first() { el(w,"note",n.value())?; }
        if let Some(ref d) = self.due_date { el(w,"due_date",&d.0.format("%Y-%m-%d").to_string())?; }
        if let Some(ref tpd) = self.tax_point_date { el(w,"tax_point_date",&tpd.0.format("%Y-%m-%d").to_string())?; }
        el_opt(w,"invoice_type_code",self.invoice_type_code.as_ref().map(|c| c.value()))?;
        el_opt(w,"document_currency_code",self.document_currency_code.as_ref().map(|c| c.value()))?;
        cac_opt(w,"AccountingSupplierParty",Some(&self.accounting_supplier_party))?;
        cac_opt(w,"AccountingCustomerParty",self.accounting_customer_party.as_ref())?;
        for pm in &self.payment_means { open(w,"PaymentMeans")?; el(w,"payment_means_code",pm.payment_means_code.value())?; close(w,"PaymentMeans")?; }
        cac_vec(w,"TaxTotal",&self.tax_total)?;
        open(w,"LegalMonetaryTotal")?; self.legal_monetary_total.to_xml(w)?; close(w,"LegalMonetaryTotal")?;
        for line in &self.invoice_line {
            open(w,"InvoiceLine")?;
            el(w,"id",line.id.value())?;
            if let Some(ref q) = line.invoiced_quantity { el(w,"invoiced_quantity",&q.value().to_string())?; }
            el(w,"line_extension_amount",&format!("{:.2}",line.line_extension_amount.value()))?;
            cac_opt(w,"Item",Some(&line.item))?;
            cac_vec(w,"TaxTotal",&line.tax_total)?;
            close(w,"InvoiceLine")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invoice_to_xml() {
        let json = r#"{
            "id": {"value": "INV-001"},
            "issue_date": "2026-06-12",
            "invoice_type_code": {"value": "380"},
            "document_currency_code": {"value": "ZAR"},
            "note": [{"value": "Thank you"}],
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
                "item": {"description": {"value": "Widget"}, "name": "Widget"}
            }]
        }"#;
        let invoice: Invoice = serde_json::from_str(json).expect("JSON");
        let xml = to_string(&invoice, "Invoice").expect("XML");
        assert!(xml.contains("<cbc:ID>INV-001</cbc:ID>"));
        assert!(xml.contains("<cbc:IssueDate>2026-06-12</cbc:IssueDate>"));
        assert!(xml.contains("<cbc:PayableAmount>115.00</cbc:PayableAmount>"));
        assert!(xml.contains("</Invoice>"));
    }
}
