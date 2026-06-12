// UBL 2.5 Catalogue Document Types
//
// Reference: https://docs.oasis-open.org/ubl/cs01-UBL-2.5/UBL-2.5.html
// Section 2.4 — Catalogue (documents 22–26)
//
// Five document types for managing catalogues of items for sale:
//   - Catalogue: A document describing items, prices, and price validity.
//   - CatalogueRequest: A document requesting a catalogue from a provider.
//   - CatalogueItemSpecificationUpdate: Updates item specifications in a catalogue.
//   - CataloguePricingUpdate: Updates prices in a catalogue.
//   - CatalogueDeletion: Cancels an entire catalogue.

use serde::{Deserialize, Serialize};
use ubl_common::cbc::*;
use ubl_common::cac::*;
use ubl_common::cac::catalogue::*;

// ── Catalogue (Document #22) ──────────────────────────────────────

/// A document that describes items, prices, and price validity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Catalogue {
    pub ubl_version_id: Option<UBLVersionID>,
    pub customization_id: Option<CustomizationID>,
    pub profile_id: Option<ProfileID>,
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    pub uuid: Option<UUID>,
    pub action_code: Option<ActionCode>,
    pub name: Option<Name>,
    pub issue_date: IssueDate,
    pub issue_time: Option<IssueTime>,
    pub revision_date: Option<RevisionDate>,
    pub revision_time: Option<RevisionTime>,
    pub note: Vec<Note>,
    pub description: Vec<Description>,
    pub version_id: Option<VersionID>,
    pub previous_version_id: Option<PreviousVersionID>,
    pub line_count_numeric: Option<LineCountNumeric>,
    pub validity_period: Vec<Period>,
    pub referenced_contract: Vec<Contract>,
    pub source_catalogue_reference: Option<CatalogueReference>,
    pub document_reference: Vec<DocumentReference>,
    pub applicable_territory_address: Vec<Address>,
    pub signature: Vec<Signature>,
    pub provider_party: Party,
    pub receiver_party: Party,
    pub seller_supplier_party: Option<SupplierParty>,
    pub contractor_customer_party: Option<CustomerParty>,
    pub trading_terms: Vec<TradingTerms>,
    pub catalogue_line: Vec<CatalogueLine>,
}

// ── CatalogueRequest (Document #23) ───────────────────────────────

/// A document used to request a Catalogue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueRequest {
    pub ubl_version_id: Option<UBLVersionID>,
    pub customization_id: Option<CustomizationID>,
    pub profile_id: Option<ProfileID>,
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    pub uuid: Option<UUID>,
    pub name: Option<Name>,
    pub issue_date: IssueDate,
    pub issue_time: Option<IssueTime>,
    pub note: Vec<Note>,
    pub description: Vec<Description>,
    pub pricing_update_request_indicator: Option<PricingUpdateRequestIndicator>,
    pub item_update_request_indicator: Option<ItemUpdateRequestIndicator>,
    pub line_count_numeric: Option<LineCountNumeric>,
    pub validity_period: Vec<Period>,
    pub signature: Vec<Signature>,
    pub receiver_party: Party,
    pub provider_party: Party,
    pub seller_supplier_party: Option<SupplierParty>,
    pub contractor_customer_party: Option<CustomerParty>,
    pub requested_catalogue_reference: Option<CatalogueReference>,
    pub referenced_contract: Vec<Contract>,
    pub trading_terms: Vec<TradingTerms>,
    pub document_reference: Vec<DocumentReference>,
    pub applicable_territory_address: Vec<Address>,
    pub requested_language: Option<Language>,
    pub requested_classification_scheme: Vec<ClassificationScheme>,
    pub catalogue_request_line: Vec<CatalogueRequestLine>,
}

// ── CatalogueItemSpecificationUpdate (Document #24) ───────────────

/// A document used to update item specifications in a catalogue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueItemSpecificationUpdate {
    pub ubl_version_id: Option<UBLVersionID>,
    pub customization_id: Option<CustomizationID>,
    pub profile_id: Option<ProfileID>,
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    pub uuid: Option<UUID>,
    pub name: Option<Name>,
    pub issue_date: IssueDate,
    pub issue_time: Option<IssueTime>,
    pub revision_date: Option<RevisionDate>,
    pub revision_time: Option<RevisionTime>,
    pub note: Vec<Note>,
    pub description: Vec<Description>,
    pub version_id: Option<VersionID>,
    pub line_count_numeric: Option<LineCountNumeric>,
    pub validity_period: Vec<Period>,
    pub related_catalogue_reference: CatalogueReference,
    pub referenced_contract: Vec<Contract>,
    pub signature: Vec<Signature>,
    pub provider_party: Party,
    pub receiver_party: Party,
    pub seller_supplier_party: Option<SupplierParty>,
    pub contractor_customer_party: Option<CustomerParty>,
    pub trading_terms: Vec<TradingTerms>,
    pub default_language: Option<Language>,
    pub catalogue_item_specification_update_line: Vec<CatalogueItemSpecificationUpdateLine>,
}

// ── CataloguePricingUpdate (Document #25) ─────────────────────────

/// A document used to update prices in a catalogue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CataloguePricingUpdate {
    pub ubl_version_id: Option<UBLVersionID>,
    pub customization_id: Option<CustomizationID>,
    pub profile_id: Option<ProfileID>,
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    pub uuid: Option<UUID>,
    pub name: Option<Name>,
    pub issue_date: IssueDate,
    pub issue_time: Option<IssueTime>,
    pub revision_date: Option<RevisionDate>,
    pub revision_time: Option<RevisionTime>,
    pub note: Vec<Note>,
    pub description: Vec<Description>,
    pub version_id: Option<VersionID>,
    pub line_count_numeric: Option<LineCountNumeric>,
    pub validity_period: Vec<Period>,
    pub related_catalogue_reference: CatalogueReference,
    pub referenced_contract: Vec<Contract>,
    pub signature: Vec<Signature>,
    pub provider_party: Party,
    pub receiver_party: Party,
    pub seller_supplier_party: Option<SupplierParty>,
    pub contractor_customer_party: Option<CustomerParty>,
    pub trading_terms: Vec<TradingTerms>,
    pub default_language: Option<Language>,
    pub catalogue_pricing_update_line: Vec<CataloguePricingUpdateLine>,
}

// ── CatalogueDeletion (Document #26) ──────────────────────────────

/// A document used to cancel an entire Catalogue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueDeletion {
    pub ubl_version_id: Option<UBLVersionID>,
    pub customization_id: Option<CustomizationID>,
    pub profile_id: Option<ProfileID>,
    pub profile_execution_id: Option<ProfileExecutionID>,
    pub id: ID,
    pub uuid: Option<UUID>,
    pub name: Option<Name>,
    pub issue_date: IssueDate,
    pub issue_time: Option<IssueTime>,
    pub effective_date: Option<EffectiveDate>,
    pub effective_time: Option<EffectiveTime>,
    pub note: Vec<Note>,
    pub version_id: Option<VersionID>,
    pub description: Vec<Description>,
    pub validity_period: Vec<Period>,
    pub deleted_catalogue_reference: CatalogueReference,
    pub referenced_contract: Vec<Contract>,
    pub signature: Vec<Signature>,
    pub receiver_party: Party,
    pub provider_party: Party,
    pub seller_supplier_party: Option<SupplierParty>,
    pub contractor_customer_party: Option<CustomerParty>,
}
