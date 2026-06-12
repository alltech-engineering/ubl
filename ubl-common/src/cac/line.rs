// UBL 2.5 CAC Tier 4: Catalogue Line, Tender Line
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── CatalogueLine ───────────────────────────────────────────────────
// XSD: CatalogueLineType
// A line in a catalogue describing a product or service

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueLine {
    pub id: String, // 1..1 required
    pub action_code: Option<String>,
    pub life_cycle_status_code: Option<String>,
    pub contract_subdivision: Option<String>,
    pub note: Vec<String>,
    pub orderable_indicator: Option<bool>,
    pub orderable_unit: Option<String>,
    pub content_unit_quantity: Option<f64>,
    pub order_quantity_increment_numeric: Option<f64>,
    pub minimum_order_quantity: Option<f64>,
    pub maximum_order_quantity: Option<f64>,
    pub warranty_information: Vec<String>,
    pub pack_level_code: Option<String>,
    // CAC: contractor_customer_party: Option<CustomerParty>
    // CAC: seller_supplier_party: Option<SupplierParty>
    // CAC: warranty_party: Option<Party>
    // CAC: warranty_validity_period: Option<Period>
    // CAC: line_validity_period: Option<Period>
    // CAC: item_comparison: Vec<ItemComparison>
    // CAC: component/accessory/required/replacement/complementary/replaced_related_item
    // CAC: required_item_location_quantity
    // CAC: document_reference: Vec<DocumentReference>
    // CAC: item: Item  // 1..1 required
    // CAC: keyword_item_property: Vec<ItemProperty>
    // CAC: call_for_tenders_line_reference
    // CAC: call_for_tenders_document_reference: Vec<DocumentReference>
}

// ─── TenderLine ──────────────────────────────────────────────────────
// XSD: TenderLineType
// A line in a tender document

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenderLine {
    pub id: Option<String>,
    pub note: Vec<String>,
    pub quantity: Option<f64>,
    pub line_extension_amount: Option<f64>,
    pub tax_inclusive_line_extension_amount: Option<f64>,
    pub total_tax_amount: Option<f64>,
    pub orderable_unit: Option<String>,
    pub content_unit_quantity: Option<f64>,
    pub order_quantity_increment_numeric: Option<f64>,
    pub minimum_order_quantity: Option<f64>,
    pub maximum_order_quantity: Option<f64>,
    pub warranty_information: Vec<String>,
    pub pack_level_code: Option<String>,
    // CAC: document_reference: Vec<DocumentReference>
    // CAC: item: Option<Item>
    // CAC: offered_item_location_quantity
    // CAC: replacement_related_item: Vec<RelatedItem>
    // CAC: warranty_party: Option<Party>
    // CAC: warranty_validity_period: Option<Period>
    // CAC: sub_tender_line: Vec<TenderLine>
    // CAC: call_for_tenders_line_reference
    // CAC: call_for_tenders_document_reference: Vec<DocumentReference>
}
