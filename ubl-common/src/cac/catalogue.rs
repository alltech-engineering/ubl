// UBL Catalogue aggregates — CatalogueReference, CatalogueLine and related types.
//
// These are ABIEs used by the 5 Catalogue document types:
//   Catalogue, CatalogueRequest, CatalogueItemSpecificationUpdate,
//   CataloguePricingUpdate, CatalogueDeletion

use serde::{Deserialize, Serialize};
use crate::cbc::*;

use crate::cac::item::Item;
use crate::cac::document::DocumentReference;
use crate::cac::party::Party;
use crate::cac::period::Period;
use crate::cac::customer::CustomerParty;
use crate::cac::supplier::SupplierParty;
use crate::cac::address::Address;

// ── CatalogueReference ────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueReference {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub revision_date: Option<LastRevisionDate>,
    pub revision_time: Option<LastRevisionTime>,
    pub note: Vec<Note>,
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
    pub note: Vec<Note>,
    pub orderable_indicator: Option<OrderableIndicator>,
    pub orderable_unit: Option<OrderableUnit>,
    pub content_unit_quantity: Option<ContentUnitQuantity>,
    pub order_quantity_increment_numeric: Option<OrderQuantityIncrementNumeric>,
    pub minimum_order_quantity: Option<MinimumOrderQuantity>,
    pub maximum_order_quantity: Option<MaximumOrderQuantity>,
    pub warranty_information: Vec<WarrantyInformation>,
    pub pack_level_code: Option<PackLevelCode>,
    pub contractor_customer_party: Option<CustomerParty>,
    pub seller_supplier_party: Option<SupplierParty>,
    pub warranty_party: Option<WarrantyParty>,
    pub warranty_validity_period: Option<Period>,
    pub line_validity_period: Option<Period>,
    pub item_comparison: Vec<ItemComparison>,
    pub component_related_item: Vec<ComponentRelatedItem>,
    pub accessory_related_item: Vec<AccessoryRelatedItem>,
    pub required_related_item: Vec<RequiredRelatedItem>,
    pub replacement_related_item: Vec<ReplacementRelatedItem>,
    pub complementary_related_item: Vec<ComplementaryRelatedItem>,
    pub replaced_related_item: Vec<ReplacedRelatedItem>,
    pub required_item_location_quantity: Vec<RequiredItemLocationQuantity>,
    pub document_reference: Vec<DocumentReference>,
    pub item: Item,
    pub keyword_item_property: Vec<KeywordItemProperty>,
    pub call_for_tenders_line_reference: Option<CallForTendersLineReference>,
    pub call_for_tenders_document_reference: Vec<CallForTendersDocumentReference>,
}

// ── CatalogueRequestLine ──────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueRequestLine {
    pub id: ID,
    pub contract_subdivision: Option<ContractSubdivision>,
    pub note: Vec<Note>,
    pub line_validity_period: Option<Period>,
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
    pub note: Vec<Note>,
    pub version_id: Option<VersionID>,
    pub modification_reason_code: Option<ModificationReasonCode>,
    pub modification_reason_description: Vec<ModificationReasonDescription>,
    pub description: Vec<Description>,
    pub validity_period: Option<Period>,
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

// ── Signature ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub id: ID,
    pub reason_code: Option<ReasonCode>,
    pub note: Vec<Note>,
    pub validation_date: Option<ValidationDate>,
    pub validation_time: Option<ValidationTime>,
    pub validator_id: Option<ValidatorID>,
    pub canonicalization_method: Option<CanonicalizationMethod>,
    pub signature_method: Option<SignatureMethod>,
    pub signatory_party: Option<SignatoryParty>,
    pub digital_signature_attachment: Option<DigitalSignatureAttachment>,
    pub original_document_reference: Option<OriginalDocumentReference>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignatoryParty {
    pub party: Option<Party>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalSignatureAttachment {
    // stub — minimal implementation
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginalDocumentReference {
    pub id: Option<ID>,
}

// ── TradingTerms ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradingTerms {
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
    pub note: Vec<Note>,
    pub name: Option<Name>,
    pub description: Vec<Description>,
    pub agency_id: Option<AgencyID>,
    pub agency_name: Option<AgencyName>,
    pub version_id: Option<VersionID>,
    pub uri: Option<URI>,
    pub scheme_uri: Option<SchemeURI>,
    pub language_id: Option<LanguageID>,
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
