// UBL XML Serializer for UBL documents (Invoice, Order, etc.).
// Uses explicit field access — no Display impl needed on domain types.

use crate::error::{Error, Result};
use crate::ns::to_element_name;
use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use std::io::Cursor;
use ubl_common::cac;
use ubl_common::cbc;
use ubl_documents::billing::Invoice;
use ubl_documents::ordering::Order;

pub fn to_string<T: ToXml>(doc: &T, doc_type: &str) -> Result<String> {
    let mut w = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
    w.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
    let mut root = BytesStart::new(doc_type);
    root.push_attribute((
        "xmlns",
        "urn:oasis:names:specification:ubl:schema:xsd:Invoice-2",
    ));
    root.push_attribute((
        "xmlns:cac",
        "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2",
    ));
    root.push_attribute((
        "xmlns:cbc",
        "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2",
    ));
    w.write_event(Event::Start(root))?;
    doc.to_xml(&mut w)?;
    w.write_event(Event::End(BytesEnd::new(doc_type)))?;
    Ok(String::from_utf8(w.into_inner().into_inner()).map_err(|e| Error::Xml(e.to_string()))?)
}

pub trait ToXml {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()>;
}

fn el(w: &mut Writer<Cursor<Vec<u8>>>, name: &str, val: &str) -> Result<()> {
    let t = format!("cbc:{}", to_element_name(name));
    w.write_event(Event::Start(BytesStart::new(t.as_str())))?;
    w.write_event(Event::Text(BytesText::new(val)))?;
    w.write_event(Event::End(BytesEnd::new(t.as_str())))?;
    Ok(())
}
fn el_opt(w: &mut Writer<Cursor<Vec<u8>>>, name: &str, val: Option<&str>) -> Result<()> {
    if let Some(v) = val {
        el(w, name, v)?;
    }
    Ok(())
}
/// Write a CBC element with XML attributes (e.g., currencyID on Amount types)
fn el_attr(
    w: &mut Writer<Cursor<Vec<u8>>>,
    name: &str,
    val: &str,
    attrs: &[(&str, &str)],
) -> Result<()> {
    let t = format!("cbc:{}", to_element_name(name));
    let mut start = BytesStart::new(t.as_str());
    for (k, v) in attrs {
        start.push_attribute((*k, *v));
    }
    w.write_event(Event::Start(start))?;
    w.write_event(Event::Text(BytesText::new(val)))?;
    w.write_event(Event::End(BytesEnd::new(t.as_str())))?;
    Ok(())
}
fn open(w: &mut Writer<Cursor<Vec<u8>>>, name: &str) -> Result<()> {
    w.write_event(Event::Start(BytesStart::new(
        format!("cac:{}", to_element_name(name)).as_str(),
    )))?;
    Ok(())
}
fn close(w: &mut Writer<Cursor<Vec<u8>>>, name: &str) -> Result<()> {
    w.write_event(Event::End(BytesEnd::new(
        format!("cac:{}", to_element_name(name)).as_str(),
    )))?;
    Ok(())
}
fn cac_opt<T: ToXml>(w: &mut Writer<Cursor<Vec<u8>>>, name: &str, val: Option<&T>) -> Result<()> {
    if let Some(v) = val {
        open(w, name)?;
        v.to_xml(w)?;
        close(w, name)?;
    }
    Ok(())
}
fn cac_vec<T: ToXml>(w: &mut Writer<Cursor<Vec<u8>>>, name: &str, vals: &[T]) -> Result<()> {
    for v in vals {
        open(w, name)?;
        v.to_xml(w)?;
        close(w, name)?;
    }
    Ok(())
}

// ── CAC ToXml impls ──

impl ToXml for cac::Party {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        for pi in &self.party_identification {
            open(w, "PartyIdentification")?;
            el(w, "id", pi.id.value())?;
            close(w, "PartyIdentification")?;
        }
        for pn in &self.party_name {
            open(w, "PartyName")?;
            el(w, "name", &pn.name.0)?;
            close(w, "PartyName")?;
        }
        cac_opt(w, "PostalAddress", self.postal_address.as_ref())?;
        cac_opt(w, "Contact", self.contact.as_ref())?;
        for pts in &self.party_tax_scheme {
            open(w, "PartyTaxScheme")?;
            el_opt(
                w,
                "registration_name",
                pts.registration_name.as_ref().map(|n| n.0.as_str()),
            )?;
            el_opt(
                w,
                "company_id",
                pts.company_id.as_ref().map(|id| id.value()),
            )?;
            open(w, "TaxScheme")?;
            el_opt(w, "id", pts.tax_scheme.id.as_ref().map(|id| id.value()))?;
            el_opt(
                w,
                "name",
                pts.tax_scheme.name.as_ref().map(|n| n.0.as_str()),
            )?;
            close(w, "TaxScheme")?;
            close(w, "PartyTaxScheme")?;
        }
        Ok(())
    }
}

impl ToXml for cac::PostalAddress {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_opt(
            w,
            "street_name",
            self.street_name.as_ref().map(|n| n.0.as_str()),
        )?;
        el_opt(
            w,
            "city_name",
            self.city_name.as_ref().map(|n| n.0.as_str()),
        )?;
        el_opt(
            w,
            "postal_zone",
            self.postal_zone.as_ref().map(|n| n.0.value.as_str()),
        )?;
        if let Some(ref c) = self.country {
            open(w, "Country")?;
            el_opt(
                w,
                "identification_code",
                c.identification_code.as_ref().map(|code| code.value()),
            )?;
            close(w, "Country")?;
        }
        Ok(())
    }
}

impl ToXml for cac::Contact {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_opt(w, "name", self.name.as_ref().map(|n| n.0.as_str()))?;
        el_opt(
            w,
            "telephone",
            self.telephone.as_ref().map(|n| n.0.value.as_str()),
        )?;
        el_opt(
            w,
            "electronic_mail",
            self.electronic_mail.as_ref().map(|t| t.value.as_str()),
        )?;
        Ok(())
    }
}

impl ToXml for cac::SupplierParty {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        cac_opt(w, "Party", self.party.as_ref())
    }
}
impl ToXml for cac::CustomerParty {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        cac_opt(w, "Party", self.party.as_ref())
    }
}

impl ToXml for cac::TaxScheme {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_opt(w, "id", self.id.as_ref().map(|id| id.value()))?;
        el_opt(w, "name", self.name.as_ref().map(|n| n.0.as_str()))?;
        Ok(())
    }
}

impl ToXml for cac::TaxCategory {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_opt(w, "id", self.id.as_ref().map(|id| id.value()))?;
        if let Some(ref p) = self.percent {
            el(w, "percent", &p.0.to_string())?;
        }
        open(w, "TaxScheme")?;
        self.tax_scheme.to_xml(w)?;
        close(w, "TaxScheme")?;
        Ok(())
    }
}

impl ToXml for cac::TaxSubtotal {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        // XSD sequence: TaxableAmount?, TaxAmount, TaxInclusiveAmount?,
        //   CalculationSequenceNumeric?, TransactionCurrencyTaxAmount?,
        //   Percent?, BaseUnitMeasure?, PerUnitAmount?, TierRange?,
        //   TierRatePercent?, TaxCategory
        if let Some(ref ta) = self.taxable_amount {
            el_attr(
                w,
                "taxable_amount",
                &format!("{:.2}", ta.value()),
                &[("currencyID", ta.currency_id())],
            )?;
        }
        el_attr(
            w,
            "tax_amount",
            &format!("{:.2}", self.tax_amount.value()),
            &[("currencyID", self.tax_amount.currency_id())],
        )?;
        if let Some(ref p) = self.percent {
            el(w, "percent", &p.0.to_string())?;
        }
        open(w, "TaxCategory")?;
        self.tax_category.to_xml(w)?;
        close(w, "TaxCategory")?;
        Ok(())
    }
}

impl ToXml for cac::TaxTotal {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_attr(
            w,
            "tax_amount",
            &format!("{:.2}", self.tax_amount.value()),
            &[("currencyID", self.tax_amount.currency_id())],
        )?;
        cac_vec(w, "TaxSubtotal", &self.tax_subtotal)?;
        Ok(())
    }
}

impl ToXml for cac::LegalTotal {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_attr(
            w,
            "line_extension_amount",
            &format!("{:.2}", self.line_extension_amount.value()),
            &[("currencyID", self.line_extension_amount.currency_id())],
        )?;
        let te = format!(
            "{:.2}",
            self.tax_exclusive_amount
                .as_ref()
                .map(|a| *a.value())
                .unwrap_or(*self.line_extension_amount.value())
        );
        el_attr(
            w,
            "tax_exclusive_amount",
            &te,
            &[("currencyID", self.line_extension_amount.currency_id())],
        )?;
        el_attr(
            w,
            "payable_amount",
            &format!("{:.2}", self.payable_amount.value()),
            &[("currencyID", self.payable_amount.currency_id())],
        )?;
        Ok(())
    }
}

impl ToXml for cac::Item {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_opt(w, "description", self.description.as_ref().map(|d| d.value()))?;
        el_opt(w, "name", self.name.as_ref().map(|n| n.0.as_str()))?;
        // SellersItemIdentification
        if let Some(ref sid) = self.sellers_item_identification {
            open(w, "SellersItemIdentification")?;
            el(w, "id", sid.id.value())?;
            close(w, "SellersItemIdentification")?;
        }
        // BuyersItemIdentification
        if let Some(ref bid) = self.buyers_item_identification {
            open(w, "BuyersItemIdentification")?;
            el(w, "id", bid.id.value())?;
            close(w, "BuyersItemIdentification")?;
        }
        // StandardItemIdentification
        if let Some(ref std) = self.standard_item_identification {
            open(w, "StandardItemIdentification")?;
            el(w, "id", std.id.value())?;
            close(w, "StandardItemIdentification")?;
        }
        // CommodityClassification
        for cc in &self.commodity_classification {
            open(w, "CommodityClassification")?;
            el(w, "item_classification_code", cc.item_classification_code.as_ref().map(|c| c.value()).unwrap_or(""))?;
            close(w, "CommodityClassification")?;
        }
        // AdditionalItemProperty (item_property field)
        for ip in &self.item_property {
            open(w, "AdditionalItemProperty")?;
            el(w, "name", ip.name.value.as_str())?;
            el(w, "value", ip.value.value.as_str())?;
            close(w, "AdditionalItemProperty")?;
        }
        Ok(())
    }
}

impl ToXml for cac::Period {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        if let Some(ref sd) = self.start_date {
            el(w, "start_date", &sd.0.format("%Y-%m-%d").to_string())?;
        }
        if let Some(ref ed) = self.end_date {
            el(w, "end_date", &ed.0.format("%Y-%m-%d").to_string())?;
        }
        Ok(())
    }
}

// ── DocumentReference helper ──
// Writes a CAC wrapper with cbc:ID inside (for OrderDocumentReference, etc.)
fn write_doc_ref(w: &mut Writer<Cursor<Vec<u8>>>, cac_name: &str, dr: &cac::DocumentReference) -> Result<()> {
    open(w, cac_name)?;
    if let Some(ref id) = dr.id {
        el(w, "id", id.value())?;
    }
    if let Some(ref uuid) = dr.uuid {
        el(w, "uuid", uuid.value())?;
    }
    if let Some(ref d) = dr.issue_date {
        el(w, "issue_date", &d.0.format("%Y-%m-%d").to_string())?;
    }
    if let Some(ref t) = dr.issue_time {
        el(w, "issue_time", &t.0.format("%H:%M:%S").to_string())?;
    }
    el_opt(w, "document_type_code", dr.document_type_code.as_ref().map(|c| c.value()))?;
    close(w, cac_name)?;
    Ok(())
}

impl ToXml for cac::Country {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_opt(
            w,
            "identification_code",
            self.identification_code.as_ref().map(|c| c.value()),
        )?;
        Ok(())
    }
}

impl ToXml for cac::Price {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        el_attr(
            w,
            "price_amount",
            &self.price_amount.value().to_string(),
            &[("currencyID", self.price_amount.currency_id())],
        )?;
        if let Some(ref bq) = self.base_quantity {
            el_attr(
                w,
                "base_quantity",
                &bq.value().to_string(),
                &[("unitCode", bq.0.unit_code.as_deref().unwrap_or(""))],
            )?;
        }
        cac_vec(w, "ValidityPeriod", &self.validity_period)?;
        Ok(())
    }
}

// ── CAC LineItem ──

impl ToXml for cac::LineItem {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        // XSD sequence: ID?, SalesOrderID?, UUID?, Note*, LineStatusCode?,
        //   Quantity?, LineExtensionAmount?, TaxInclusiveLineExtensionAmount?,
        //   TotalTaxAmount?, MinimumQuantity?, MaximumQuantity?,
        //   MinimumBackorderQuantity?, MaximumBackorderQuantity?,
        //   InspectionMethodCode?, PartialDeliveryIndicator?,
        //   BackOrderAllowedIndicator?, AccountingCostCode?, AccountingCost?,
        //   WarrantyInformation*, Delivery*, DeliveryTerms?,
        //   OriginatorParty?, BeneficiaryParty?, OrderedShipment*,
        //   PricingReference?, AllowanceCharge*, Price?, Item?,
        //   SubLineItem*, WarrantyValidityPeriod?, WarrantyParty?,
        //   TaxTotal?, ItemPriceExtension*, LineReference*
        el_opt(w, "id", self.id.as_ref().map(|id| id.value()))?;
        el_opt(w, "sales_order_id", self.sales_order_id.as_ref().map(|id| id.value()))?;
        el_opt(w, "uuid", self.uuid.as_ref().map(|id| id.value()))?;
        for n in &self.note {
            el(w, "note", n.value())?;
        }
        el_opt(w, "line_status_code", self.line_status_code.as_ref().map(|c| c.value()))?;
        if let Some(ref q) = self.quantity {
            el_attr(
                w,
                "quantity",
                &q.value.to_string(),
                &[("unitCode", q.unit_code.as_deref().unwrap_or(""))],
            )?;
        }
        if let Some(ref a) = self.line_extension_amount {
            el_attr(
                w,
                "line_extension_amount",
                &format!("{:.2}", a.value),
                &[("currencyID", &a.currency_id)],
            )?;
        }
        if let Some(ref a) = self.tax_inclusive_line_extension_amount {
            el_attr(
                w,
                "tax_inclusive_line_extension_amount",
                &format!("{:.2}", a.value),
                &[("currencyID", &a.currency_id)],
            )?;
        }
        if let Some(ref a) = self.total_tax_amount {
            el_attr(
                w,
                "total_tax_amount",
                &format!("{:.2}", a.value),
                &[("currencyID", &a.currency_id)],
            )?;
        }
        // MinimumQuantity, MaximumQuantity, etc.
        if let Some(ref q) = self.minimum_quantity {
            el_attr(w, "minimum_quantity", &q.value.to_string(), &[("unitCode", q.unit_code.as_deref().unwrap_or(""))])?;
        }
        if let Some(ref q) = self.maximum_quantity {
            el_attr(w, "maximum_quantity", &q.value.to_string(), &[("unitCode", q.unit_code.as_deref().unwrap_or(""))])?;
        }
        if let Some(ref q) = self.minimum_backorder_quantity {
            el_attr(w, "minimum_backorder_quantity", &q.value.to_string(), &[("unitCode", q.unit_code.as_deref().unwrap_or(""))])?;
        }
        if let Some(ref q) = self.maximum_backorder_quantity {
            el_attr(w, "maximum_backorder_quantity", &q.value.to_string(), &[("unitCode", q.unit_code.as_deref().unwrap_or(""))])?;
        }
        el_opt(w, "inspection_method_code", self.inspection_method_code.as_ref().map(|c| c.value()))?;
        if let Some(ref ind) = self.partial_delivery_indicator {
            el(w, "partial_delivery_indicator", if ind.0 { "true" } else { "false" })?;
        }
        if let Some(ref ind) = self.back_order_allowed_indicator {
            el(w, "back_order_allowed_indicator", if ind.0 { "true" } else { "false" })?;
        }
        el_opt(w, "accounting_cost_code", self.accounting_cost_code.as_ref().map(|c| c.value()))?;
        el_opt(w, "accounting_cost", self.accounting_cost.as_ref().map(|c| c.value()))?;
        for wi in &self.warranty_information {
            el(w, "warranty_information", wi.0.value.as_str())?;
        }
        // Delivery*, DeliveryTerms?, etc. — skip complex sub-types for now
        cac_opt(w, "OriginatorParty", self.originator_party.as_ref())?;
        cac_opt(w, "BeneficiaryParty", self.beneficiary_party.as_ref())?;
        // AllowanceCharge* — skip for now
        cac_opt(w, "Price", self.price.as_ref())?;
        cac_opt(w, "Item", self.item.as_ref())?;
        // SubLineItem* — skip for now
        cac_opt(w, "WarrantyValidityPeriod", self.warranty_validity_period.as_ref())?;
        cac_opt(w, "WarrantyParty", self.warranty_party.as_ref())?;
        cac_opt(w, "TaxTotal", self.tax_total.as_ref())?;
        Ok(())
    }
}

// ── CAC OrderLine ──

impl ToXml for cac::OrderLine {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        // XSD sequence: SubstitutionStatusCode?, Note*, LineItem?,
        //   SellerProposedSubstituteLineItem?, SellerSubstitutedLineItem?,
        //   BuyerProposedSubstituteLineItem?, CatalogueLineReference*,
        //   QuotationLineReference*, OrderLineReference*, DocumentReference*
        el_opt(
            w,
            "substitution_status_code",
            self.substitution_status_code.as_ref().map(|c| c.value()),
        )?;
        for n in &self.note {
            el(w, "note", n.value())?;
        }
        cac_opt(w, "LineItem", self.line_item.as_ref())?;
        cac_opt(
            w,
            "SellerProposedSubstituteLineItem",
            self.seller_proposed_substitute_line_item.as_ref(),
        )?;
        cac_opt(
            w,
            "SellerSubstitutedLineItem",
            self.seller_substituted_line_item.as_ref(),
        )?;
        cac_opt(
            w,
            "BuyerProposedSubstituteLineItem",
            self.buyer_proposed_substitute_line_item.as_ref(),
        )?;
        // Line references — write minimal: open wrapper, el line_id
        for clr in &self.catalogue_line_reference {
            open(w, "CatalogueLineReference")?;
            el(w, "line_id", clr.line_id.value())?;
            close(w, "CatalogueLineReference")?;
        }
        for qlr in &self.quotation_line_reference {
            open(w, "QuotationLineReference")?;
            el(w, "line_id", qlr.line_id.value())?;
            close(w, "QuotationLineReference")?;
        }
        for olr in &self.order_line_reference {
            open(w, "OrderLineReference")?;
            el(w, "line_id", olr.line_id.value())?;
            close(w, "OrderLineReference")?;
        }
        for dr in &self.document_reference {
            write_doc_ref(w, "DocumentReference", dr)?;
        }
        Ok(())
    }
}

// ── Document: Order ──

impl ToXml for Order {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        // XSD ORDER SEQUENCE (UBL 2.5):
        // 1. cbc:UBLVersionID?, cbc:CustomizationID?, cbc:ProfileID?, cbc:ProfileExecutionID?
        el_opt(w, "ubl_version_id", self.ubl_version_id.as_ref().map(|v| v.value()))?;
        el_opt(w, "customization_id", self.customization_id.as_ref().map(|v| v.value()))?;
        el_opt(w, "profile_id", self.profile_id.as_ref().map(|v| v.value()))?;
        el_opt(w, "profile_execution_id", self.profile_execution_id.as_ref().map(|v| v.value()))?;
        // 2. cbc:ID, cbc:SalesOrderID?, cbc:UUID?
        el(w, "id", self.id.value())?;
        el_opt(w, "sales_order_id", self.sales_order_id.as_ref().map(|v| v.value()))?;
        el_opt(w, "uuid", self.uuid.as_ref().map(|v| v.value()))?;
        // 3. cbc:IssueDate, cbc:IssueTime?, cbc:OrderTypeCode?
        el(w, "issue_date", &self.issue_date.0.format("%Y-%m-%d").to_string())?;
        if let Some(ref t) = self.issue_time {
            el(w, "issue_time", &t.0.format("%H:%M:%S").to_string())?;
        }
        el_opt(w, "order_type_code", self.order_type_code.as_ref().map(|c| c.value()))?;
        // 4. cbc:Note*
        for n in &self.note {
            el(w, "note", n.value())?;
        }
        // 5. cbc:RequestedInvoiceCurrencyCode?, cbc:DocumentCurrencyCode?, cbc:PricingCurrencyCode?, cbc:TaxCurrencyCode?
        el_opt(w, "requested_invoice_currency_code", self.requested_invoice_currency_code.as_ref().map(|c| c.value()))?;
        el_opt(w, "document_currency_code", self.document_currency_code.as_ref().map(|c| c.value()))?;
        el_opt(w, "pricing_currency_code", self.pricing_currency_code.as_ref().map(|c| c.value()))?;
        el_opt(w, "tax_currency_code", self.tax_currency_code.as_ref().map(|c| c.value()))?;
        // 6. cbc:CustomerReference?, cbc:AccountingCostCode?, cbc:AccountingCost?, cbc:LineCountNumeric?
        el_opt(w, "customer_reference", self.customer_reference.as_ref().map(|c| c.value()))?;
        el_opt(w, "accounting_cost_code", self.accounting_cost_code.as_ref().map(|c| c.value()))?;
        el_opt(w, "accounting_cost", self.accounting_cost.as_ref().map(|c| c.value()))?;
        if let Some(ref lcn) = self.line_count_numeric {
            el(w, "line_count_numeric", &lcn.value().to_string())?;
        }
        // 7. cac:ValidityPeriod*, cac:QuotationDocumentReference?, cac:OrderDocumentReference*,
        //    cac:OriginatorDocumentReference?, cac:CatalogueReference?, cac:AdditionalDocumentReference*
        cac_vec(w, "ValidityPeriod", &self.validity_period)?;
        if let Some(ref qdr) = self.quotation_document_reference {
            write_doc_ref(w, "QuotationDocumentReference", qdr)?;
        }
        for odr in &self.order_document_reference {
            write_doc_ref(w, "OrderDocumentReference", odr)?;
        }
        if let Some(ref odr) = self.originator_document_reference {
            write_doc_ref(w, "OriginatorDocumentReference", odr)?;
        }
        if let Some(ref cr) = self.catalogue_reference {
            open(w, "CatalogueReference")?;
            el(w, "id", cr.id.value())?;
            el_opt(w, "uuid", cr.uuid.as_ref().map(|v| v.value()))?;
            if let Some(ref d) = cr.issue_date {
                el(w, "issue_date", &d.0.format("%Y-%m-%d").to_string())?;
            }
            close(w, "CatalogueReference")?;
        }
        for adr in &self.additional_document_reference {
            write_doc_ref(w, "AdditionalDocumentReference", adr)?;
        }
        // 8. cac:Contract*, cac:Signature*
        for c in &self.contract {
            open(w, "Contract")?;
            el_opt(w, "id", c.id.as_ref().map(|id| id.value()))?;
            if let Some(ref d) = c.issue_date {
                el(w, "issue_date", &d.0.format("%Y-%m-%d").to_string())?;
            }
            if let Some(ref t) = c.issue_time {
                el(w, "issue_time", &t.0.format("%H:%M:%S").to_string())?;
            }
            el_opt(w, "contract_type_code", c.contract_type_code.as_ref().map(|c| c.value()))?;
            el_opt(w, "contract_type", c.contract_type.as_ref().map(|t| t.value()))?;
            close(w, "Contract")?;
        }
        for s in &self.signature {
            open(w, "Signature")?;
            el(w, "id", s.id.value())?;
            for n in &s.note {
                el(w, "note", n.value())?;
            }
            if let Some(ref d) = s.validation_date {
                el(w, "validation_date", &d.0.format("%Y-%m-%d").to_string())?;
            }
            if let Some(ref t) = s.validation_time {
                el(w, "validation_time", &t.0.format("%H:%M:%S").to_string())?;
            }
            el_opt(w, "validator_id", s.validator_id.as_ref().map(|v| v.value()))?;
            el_opt(w, "signature_method_code", s.signature_method_code.as_ref().map(|c| c.value()))?;
            cac_opt(w, "SignatoryParty", s.signatory_party.as_ref())?;
            close(w, "Signature")?;
        }
        // 9. cac:BuyerCustomerParty, cac:SellerSupplierParty
        open(w, "BuyerCustomerParty")?;
        self.buyer_customer_party.to_xml(w)?;
        close(w, "BuyerCustomerParty")?;
        open(w, "SellerSupplierParty")?;
        self.seller_supplier_party.to_xml(w)?;
        close(w, "SellerSupplierParty")?;
        // 10. cac:OriginatorCustomerParty?, cac:FreightForwarderParty?,
        //     cac:AccountingCustomerParty?, cac:AccountingSupplierParty?
        cac_opt(w, "OriginatorCustomerParty", self.originator_customer_party.as_ref())?;
        cac_opt(w, "FreightForwarderParty", self.freight_forwarder_party.as_ref())?;
        cac_opt(w, "AccountingCustomerParty", self.accounting_customer_party.as_ref())?;
        cac_opt(w, "AccountingSupplierParty", self.accounting_supplier_party.as_ref())?;
        // 11. cac:Delivery*, cac:DeliveryTerms*
        for del in &self.delivery {
            open(w, "Delivery")?;
            el_opt(w, "id", del.id.as_ref().map(|id| id.value()))?;
            if let Some(ref q) = del.quantity {
                el_attr(w, "quantity", &q.value.to_string(), &[("unitCode", q.unit_code.as_deref().unwrap_or(""))])?;
            }
            if let Some(ref d) = del.actual_delivery_date {
                el(w, "actual_delivery_date", &d.0.format("%Y-%m-%d").to_string())?;
            }
            // Skip DeliveryAddress — Address doesn't have ToXml impl yet
            close(w, "Delivery")?;
        }
        for dt in &self.delivery_terms {
            open(w, "DeliveryTerms")?;
            el_opt(w, "id", dt.id.as_ref().map(|id| id.value()))?;
            for st in &dt.special_terms {
                el(w, "special_terms", st.value.as_str())?;
            }
            close(w, "DeliveryTerms")?;
        }
        // 12. cac:PaymentMeans*, cac:PaymentTerms*
        for pm in &self.payment_means {
            open(w, "PaymentMeans")?;
            el(w, "payment_means_code", pm.payment_means_code.value())?;
            if let Some(ref d) = pm.payment_due_date {
                el(w, "payment_due_date", &d.0.format("%Y-%m-%d").to_string())?;
            }
            close(w, "PaymentMeans")?;
        }
        for pt in &self.payment_terms {
            open(w, "PaymentTerms")?;
            el_opt(w, "id", pt.id.as_ref().map(|id| id.value()))?;
            for n in &pt.note {
                el(w, "note", n.value())?;
            }
            close(w, "PaymentTerms")?;
        }
        // 13. cac:TransactionConditions?
        if let Some(ref tc) = self.transaction_conditions {
            open(w, "TransactionConditions")?;
            el_opt(w, "id", tc.id.as_ref().map(|id| id.value()))?;
            el_opt(w, "action_code", tc.action_code.as_ref().map(|c| c.value()))?;
            for desc in &tc.description {
                el(w, "description", desc.value())?;
            }
            close(w, "TransactionConditions")?;
        }
        // 14. cac:AllowanceCharge*
        for ac in &self.allowance_charge {
            open(w, "AllowanceCharge")?;
            let ci = &ac.charge_indicator;
            el(w, "charge_indicator", if ci.0 { "true" } else { "false" })?;
            el_attr(w, "amount", &format!("{:.2}", ac.amount.value), &[("currencyID", &ac.amount.currency_id)])?;
            if let Some(ref ba) = ac.base_amount {
                el_attr(w, "base_amount", &format!("{:.2}", ba.0.value), &[("currencyID", &ba.0.currency_id)])?;
            }
            el_opt(w, "multiplier_factor_numeric", ac.multiplier_factor_numeric.as_ref().map(|m| m.value().to_string()).as_deref().as_deref())?;
            close(w, "AllowanceCharge")?;
        }
        // 15. cac:TaxExchangeRate?, cac:PricingExchangeRate?, cac:PaymentExchangeRate?
        if let Some(ref er) = self.tax_exchange_rate {
            open(w, "TaxExchangeRate")?;
            el(w, "source_currency_code", er.source_currency_code.value())?;
            el(w, "target_currency_code", er.target_currency_code.value())?;
            if let Some(ref cr) = er.calculation_rate {
                el(w, "calculation_rate", &cr.value().to_string())?;
            }
            close(w, "TaxExchangeRate")?;
        }
        if let Some(ref er) = self.pricing_exchange_rate {
            open(w, "PricingExchangeRate")?;
            el(w, "source_currency_code", er.source_currency_code.value())?;
            el(w, "target_currency_code", er.target_currency_code.value())?;
            if let Some(ref cr) = er.calculation_rate {
                el(w, "calculation_rate", &cr.value().to_string())?;
            }
            close(w, "PricingExchangeRate")?;
        }
        if let Some(ref er) = self.payment_exchange_rate {
            open(w, "PaymentExchangeRate")?;
            el(w, "source_currency_code", er.source_currency_code.value())?;
            el(w, "target_currency_code", er.target_currency_code.value())?;
            if let Some(ref cr) = er.calculation_rate {
                el(w, "calculation_rate", &cr.value().to_string())?;
            }
            close(w, "PaymentExchangeRate")?;
        }
        // 16. cac:DestinationCountry?, cac:TaxTotal*
        cac_opt(w, "DestinationCountry", self.destination_country.as_ref())?;
        cac_vec(w, "TaxTotal", &self.tax_total)?;
        // 17. cac:AnticipatedMonetaryTotal?
        if let Some(ref mt) = self.anticipated_monetary_total {
            open(w, "AnticipatedMonetaryTotal")?;
            if let Some(ref lea) = mt.line_extension_amount {
                el_attr(w, "line_extension_amount", &format!("{:.2}", lea.value()), &[("currencyID", lea.currency_id())])?;
            }
            el_attr(w, "payable_amount", &format!("{:.2}", mt.payable_amount.value()), &[("currencyID", mt.payable_amount.currency_id())])?;
            close(w, "AnticipatedMonetaryTotal")?;
        }
        // 18. cac:OrderLine*
        cac_vec(w, "OrderLine", &self.order_line)?;
        // 19. cac:ProjectReference*, cac:BeneficiaryParty*
        for pr in &self.project_reference {
            open(w, "ProjectReference")?;
            el(w, "id", pr.id.value())?;
            el_opt(w, "uuid", pr.uuid.as_ref().map(|v| v.value()))?;
            if let Some(ref d) = pr.issue_date {
                el(w, "issue_date", &d.0.format("%Y-%m-%d").to_string())?;
            }
            close(w, "ProjectReference")?;
        }
        cac_vec(w, "BeneficiaryParty", &self.beneficiary_party)?;
        Ok(())
    }
}

// ── Document: Invoice ──

impl ToXml for Invoice {
    fn to_xml(&self, w: &mut Writer<Cursor<Vec<u8>>>) -> Result<()> {
        // XSD sequence: ID, IssueDate, DueDate?, InvoiceTypeCode?,
        //   Note*, TaxPointDate?, DocumentCurrencyCode?,
        //   AccountingSupplierParty, AccountingCustomerParty?,
        //   PaymentMeans*, TaxTotal*, LegalMonetaryTotal, InvoiceLine*
        el(w, "id", self.id.value())?;
        el(
            w,
            "issue_date",
            &self.issue_date.0.format("%Y-%m-%d").to_string(),
        )?;
        if let Some(ref d) = self.due_date {
            el(w, "due_date", &d.0.format("%Y-%m-%d").to_string())?;
        }
        el_opt(
            w,
            "invoice_type_code",
            self.invoice_type_code.as_ref().map(|c| c.value()),
        )?;
        for n in &self.note {
            el(w, "note", n.value())?;
        }
        if let Some(ref tpd) = self.tax_point_date {
            el(w, "tax_point_date", &tpd.0.format("%Y-%m-%d").to_string())?;
        }
        el_opt(
            w,
            "document_currency_code",
            self.document_currency_code.as_ref().map(|c| c.value()),
        )?;
        cac_opt(
            w,
            "AccountingSupplierParty",
            Some(&self.accounting_supplier_party),
        )?;
        cac_opt(
            w,
            "AccountingCustomerParty",
            self.accounting_customer_party.as_ref(),
        )?;
        for pm in &self.payment_means {
            open(w, "PaymentMeans")?;
            el(w, "payment_means_code", pm.payment_means_code.value())?;
            close(w, "PaymentMeans")?;
        }
        cac_vec(w, "TaxTotal", &self.tax_total)?;
        open(w, "LegalMonetaryTotal")?;
        self.legal_monetary_total.to_xml(w)?;
        close(w, "LegalMonetaryTotal")?;
        for line in &self.invoice_line {
            open(w, "InvoiceLine")?;
            el(w, "id", line.id.value())?;
            if let Some(ref q) = line.invoiced_quantity {
                el(w, "invoiced_quantity", &q.value().to_string())?;
            }
            el_attr(
                w,
                "line_extension_amount",
                &format!("{:.2}", line.line_extension_amount.value()),
                &[("currencyID", line.line_extension_amount.currency_id())],
            )?;
            cac_opt(w, "Item", Some(&line.item))?;
            cac_vec(w, "TaxTotal", &line.tax_total)?;
            close(w, "InvoiceLine")?;
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
        assert!(xml.contains(">115.00</cbc:PayableAmount>"));
        assert!(xml.contains("</Invoice>"));
    }
}
