// UBL Catalogue aggregates — CatalogueReference, CatalogueLine and related types.
//
// These are ABIEs used by the 5 Catalogue document types:
//   Catalogue, CatalogueRequest, CatalogueItemSpecificationUpdate,
//   CataloguePricingUpdate, CatalogueDeletion

use crate::cbc::*;
use serde::{Deserialize, Serialize};

use crate::cac::address::Address;
use crate::cac::customer::CustomerParty;
use crate::cac::document::DocumentReference;
use crate::cac::item::Item;
use crate::cac::party::Party;
use crate::cac::period::Period;
use crate::cac::price::Price;
use crate::cac::supplier::SupplierParty;

// ── CatalogueReference ────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueReference {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub revision_date: Option<LastRevisionDate>,
    pub revision_time: Option<LastRevisionTime>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub description: Vec<Description>,
    pub version_id: Option<VersionID>,
    pub previous_version_id: Option<PreviousVersionID>,
}

// ── CatalogueLine ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueLine {
    pub id: ID,
    pub action_code: Option<ActionCode>,
    pub life_cycle_status_code: Option<LifeCycleStatusCode>,
    pub contract_subdivision: Option<ContractSubdivision>,
    #[serde(default)]
    pub note: Vec<Note>,
    pub orderable_indicator: Option<OrderableIndicator>,
    pub orderable_unit: Option<OrderableUnit>,
    pub content_unit_quantity: Option<ContentUnitQuantity>,
    pub order_quantity_increment_numeric: Option<OrderQuantityIncrementNumeric>,
    pub minimum_order_quantity: Option<MinimumOrderQuantity>,
    pub maximum_order_quantity: Option<MaximumOrderQuantity>,
    #[serde(default)]
    pub warranty_information: Vec<WarrantyInformation>,
    pub pack_level_code: Option<PackLevelCode>,
    pub contractor_customer_party: Option<CustomerParty>,
    pub seller_supplier_party: Option<SupplierParty>,
    pub warranty_party: Option<WarrantyParty>,
    pub warranty_validity_period: Option<Period>,
    pub line_validity_period: Option<Period>,
    #[serde(default)]
    pub item_comparison: Vec<ItemComparison>,
    #[serde(default)]
    pub component_related_item: Vec<ComponentRelatedItem>,
    #[serde(default)]
    pub accessory_related_item: Vec<AccessoryRelatedItem>,
    #[serde(default)]
    pub required_related_item: Vec<RequiredRelatedItem>,
    #[serde(default)]
    pub replacement_related_item: Vec<ReplacementRelatedItem>,
    #[serde(default)]
    pub complementary_related_item: Vec<ComplementaryRelatedItem>,
    #[serde(default)]
    pub replaced_related_item: Vec<ReplacedRelatedItem>,
    #[serde(default)]
    pub required_item_location_quantity: Vec<RequiredItemLocationQuantity>,
    #[serde(default)]
    pub document_reference: Vec<DocumentReference>,
    pub item: Item,
    pub price: Option<Price>,
    #[serde(default)]
    pub keyword_item_property: Vec<KeywordItemProperty>,
    pub call_for_tenders_line_reference: Option<CallForTendersLineReference>,
    #[serde(default)]
    pub call_for_tenders_document_reference: Vec<CallForTendersDocumentReference>,
}

// ── CatalogueRequestLine ──────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueRequestLine {
    pub id: ID,
    pub contract_subdivision: Option<ContractSubdivision>,
    #[serde(default)]
    pub note: Vec<Note>,
    pub line_validity_period: Option<Period>,
    #[serde(default)]
    pub required_item_location_quantity: Vec<RequiredItemLocationQuantity>,
    pub item: Item,
}

// ── CatalogueItemSpecificationUpdateLine ──────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueItemSpecificationUpdateLine {
    pub id: ID,
    pub contractor_customer_party: Option<CustomerParty>,
    pub seller_supplier_party: Option<SupplierParty>,
    pub item: Item,
}

// ── CataloguePricingUpdateLine ────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CataloguePricingUpdateLine {
    pub id: ID,
    pub contractor_customer_party: Option<CustomerParty>,
    pub seller_supplier_party: Option<SupplierParty>,
    #[serde(default)]
    pub required_item_location_quantity: Vec<RequiredItemLocationQuantity>,
}

// ── Contract ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    pub id: Option<ID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub nomination_date: Option<NominationDate>,
    pub nomination_time: Option<NominationTime>,
    pub contract_type_code: Option<ContractTypeCode>,
    pub contract_type: Option<ContractType>,
    #[serde(default)]
    pub note: Vec<Note>,
    pub version_id: Option<VersionID>,
    pub modification_reason_code: Option<ModificationReasonCode>,
    #[serde(default)]
    pub modification_reason_description: Vec<ModificationReasonDescription>,
    #[serde(default)]
    pub description: Vec<Description>,
    pub validity_period: Option<Period>,
    #[serde(default)]
    pub contract_document_reference: Vec<ContractDocumentReference>,
    pub nomination_period: Option<NominationPeriod>,
    pub contractual_delivery: Option<ContractualDelivery>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractDocumentReference {
    pub id: ID,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NominationPeriod {
    pub start_date: Option<StartDate>,
    pub end_date: Option<EndDate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractualDelivery {
    // stub — minimal implementation
}

// ── TradingTerms ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradingTerms {
    #[serde(default)]
    pub information: Vec<Information>,
    pub reference: Option<Reference>,
    pub applicable_address: Option<Address>,
}

// ── ClassificationScheme ──────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassificationScheme {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub last_revision_date: Option<LastRevisionDate>,
    pub last_revision_time: Option<LastRevisionTime>,
    #[serde(default)]
    pub note: Vec<Note>,
    pub name: Option<Name>,
    #[serde(default)]
    pub description: Vec<Description>,
    pub agency_id: Option<AgencyID>,
    pub agency_name: Option<AgencyName>,
    pub version_id: Option<VersionID>,
    pub uri: Option<URI>,
    pub scheme_uri: Option<SchemeURI>,
    pub language_id: Option<LanguageID>,
    #[serde(default)]
    pub classification_category: Vec<ClassificationCategory>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassificationCategory {
    pub id: Option<ID>,
    pub name: Option<Name>,
}

// ── Placeholder stubs for CatalogueLine dependencies ──────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarrantyParty {
    // stub
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemComparison {
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentRelatedItem {
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessoryRelatedItem {
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequiredRelatedItem {
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplacementRelatedItem {
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplementaryRelatedItem {
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplacedRelatedItem {
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequiredItemLocationQuantity {
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeywordItemProperty {
    pub id: Option<ID>,
    pub name: Option<Name>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallForTendersLineReference {
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallForTendersDocumentReference {
    pub id: Option<ID>,
}
