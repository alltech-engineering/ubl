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
    #[serde(default)]
    pub uuid: Option<UUID>,
    #[serde(default)]
    pub issue_date: Option<IssueDate>,
    #[serde(default)]
    pub issue_time: Option<IssueTime>,
    #[serde(default)]
    pub revision_date: Option<LastRevisionDate>,
    #[serde(default)]
    pub revision_time: Option<LastRevisionTime>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub description: Vec<Description>,
    #[serde(default)]
    pub version_id: Option<VersionID>,
    #[serde(default)]
    pub previous_version_id: Option<PreviousVersionID>,
}

// ── CatalogueLine ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueLine {
    pub id: ID,
    #[serde(default)]
    pub action_code: Option<ActionCode>,
    #[serde(default)]
    pub life_cycle_status_code: Option<LifeCycleStatusCode>,
    #[serde(default)]
    pub contract_subdivision: Option<ContractSubdivision>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub orderable_indicator: Option<OrderableIndicator>,
    #[serde(default)]
    pub orderable_unit: Option<OrderableUnit>,
    #[serde(default)]
    pub content_unit_quantity: Option<ContentUnitQuantity>,
    #[serde(default)]
    pub order_quantity_increment_numeric: Option<OrderQuantityIncrementNumeric>,
    #[serde(default)]
    pub minimum_order_quantity: Option<MinimumOrderQuantity>,
    #[serde(default)]
    pub maximum_order_quantity: Option<MaximumOrderQuantity>,
    #[serde(default)]
    pub warranty_information: Vec<WarrantyInformation>,
    #[serde(default)]
    pub pack_level_code: Option<PackLevelCode>,
    #[serde(default)]
    pub contractor_customer_party: Option<CustomerParty>,
    #[serde(default)]
    pub seller_supplier_party: Option<SupplierParty>,
    #[serde(default)]
    pub warranty_party: Option<WarrantyParty>,
    #[serde(default)]
    pub warranty_validity_period: Option<Period>,
    #[serde(default)]
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
    #[serde(default)]
    pub price: Option<Price>,
    #[serde(default)]
    pub keyword_item_property: Vec<KeywordItemProperty>,
    #[serde(default)]
    pub call_for_tenders_line_reference: Option<CallForTendersLineReference>,
    #[serde(default)]
    pub call_for_tenders_document_reference: Vec<CallForTendersDocumentReference>,
}

// ── CatalogueRequestLine ──────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueRequestLine {
    pub id: ID,
    #[serde(default)]
    pub contract_subdivision: Option<ContractSubdivision>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub line_validity_period: Option<Period>,
    #[serde(default)]
    pub required_item_location_quantity: Vec<RequiredItemLocationQuantity>,
    pub item: Item,
}

// ── CatalogueItemSpecificationUpdateLine ──────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueItemSpecificationUpdateLine {
    pub id: ID,
    #[serde(default)]
    pub contractor_customer_party: Option<CustomerParty>,
    #[serde(default)]
    pub seller_supplier_party: Option<SupplierParty>,
    pub item: Item,
}

// ── CataloguePricingUpdateLine ────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CataloguePricingUpdateLine {
    pub id: ID,
    #[serde(default)]
    pub contractor_customer_party: Option<CustomerParty>,
    #[serde(default)]
    pub seller_supplier_party: Option<SupplierParty>,
    #[serde(default)]
    pub required_item_location_quantity: Vec<RequiredItemLocationQuantity>,
}

// ── Contract ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub issue_date: Option<IssueDate>,
    #[serde(default)]
    pub issue_time: Option<IssueTime>,
    #[serde(default)]
    pub nomination_date: Option<NominationDate>,
    #[serde(default)]
    pub nomination_time: Option<NominationTime>,
    #[serde(default)]
    pub contract_type_code: Option<ContractTypeCode>,
    #[serde(default)]
    pub contract_type: Option<ContractType>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub version_id: Option<VersionID>,
    #[serde(default)]
    pub modification_reason_code: Option<ModificationReasonCode>,
    #[serde(default)]
    pub modification_reason_description: Vec<ModificationReasonDescription>,
    #[serde(default)]
    pub description: Vec<Description>,
    #[serde(default)]
    pub validity_period: Option<Period>,
    #[serde(default)]
    pub contract_document_reference: Vec<ContractDocumentReference>,
    #[serde(default)]
    pub nomination_period: Option<NominationPeriod>,
    #[serde(default)]
    pub contractual_delivery: Option<ContractualDelivery>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractDocumentReference {
    pub id: ID,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NominationPeriod {
    #[serde(default)]
    pub start_date: Option<StartDate>,
    #[serde(default)]
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
    #[serde(default)]
    pub reference: Option<Reference>,
    #[serde(default)]
    pub applicable_address: Option<Address>,
}

// ── ClassificationScheme ──────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassificationScheme {
    pub id: ID,
    #[serde(default)]
    pub uuid: Option<UUID>,
    #[serde(default)]
    pub last_revision_date: Option<LastRevisionDate>,
    #[serde(default)]
    pub last_revision_time: Option<LastRevisionTime>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub name: Option<Name>,
    #[serde(default)]
    pub description: Vec<Description>,
    #[serde(default)]
    pub agency_id: Option<AgencyID>,
    #[serde(default)]
    pub agency_name: Option<AgencyName>,
    #[serde(default)]
    pub version_id: Option<VersionID>,
    #[serde(default)]
    pub uri: Option<URI>,
    #[serde(default)]
    pub scheme_uri: Option<SchemeURI>,
    #[serde(default)]
    pub language_id: Option<LanguageID>,
    #[serde(default)]
    pub classification_category: Vec<ClassificationCategory>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClassificationCategory {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub name: Option<Name>,
}

// ── Placeholder stubs for CatalogueLine dependencies ──────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarrantyParty {
    // stub
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemComparison {
    #[serde(default)]
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentRelatedItem {
    #[serde(default)]
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessoryRelatedItem {
    #[serde(default)]
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequiredRelatedItem {
    #[serde(default)]
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplacementRelatedItem {
    #[serde(default)]
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplementaryRelatedItem {
    #[serde(default)]
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplacedRelatedItem {
    #[serde(default)]
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequiredItemLocationQuantity {
    #[serde(default)]
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeywordItemProperty {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub name: Option<Name>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallForTendersLineReference {
    #[serde(default)]
    pub id: Option<ID>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallForTendersDocumentReference {
    #[serde(default)]
    pub id: Option<ID>,
}
