// UBL 2.5 Transportation & Logistics Document Types
//
// 16 document types: BillOfLading through CommonTransportationReport
// Reference: https://docs.oasis-open.org/ubl/cs01-UBL-2.5/xsd/maindoc/

use serde::{Deserialize, Serialize};

// CBC types
use ubl_common::cbc::*;

// CAC types — declared modules
use ubl_common::cac::delivery::{GoodsItem, Shipment};
use ubl_common::cac::document::{DocumentDistribution, DocumentReference, Signature};
use ubl_common::cac::exchange_rate::{Contract as ExchangeRateContract, ExchangeRate};
use ubl_common::cac::party::{Location, Party};
use ubl_common::cac::payment::PaymentTerms as PaymentTermsType;
use ubl_common::cac::period::Period;
use ubl_common::cac::transport::{
    CertificateOfOriginApplication, Consignment, Endorsement, GoodsItemPassportCounterfoil,
    Package, TransportEquipment, TransportEvent, TransportExecutionTerms, TransportMeans,
    TransportSchedule, TransportationSegment, TransportationService,
};

// ─── BillOfLading ──────────────────────────────────────

/// UBL 2.5 BillOfLading document.
///
/// Reference: UBL-BillOfLading-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BillOfLading {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub name: Option<Name>,
    pub description: Vec<Description>,
    pub note: Vec<Note>,
    pub status_reason_code: Option<Code>,
    pub consignor_party: Option<Party>,
    pub carrier_party: Option<Party>,
    pub freight_forwarder_party: Option<Party>,
    pub shipment: Option<Shipment>,
    pub document_reference: Vec<DocumentReference>,
    pub exchange_rate: Vec<ExchangeRate>,
    pub document_distribution: Vec<DocumentDistribution>,
    pub signature: Vec<Signature>,
}

// ─── Waybill ──────────────────────────────────────

/// UBL 2.5 Waybill document.
///
/// Reference: UBL-Waybill-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Waybill {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub name: Option<Name>,
    pub description: Vec<Description>,
    pub note: Vec<Note>,
    pub waybill_type_code: Option<Code>,
    pub consolidated_indicator: Option<Indicator>,
    pub sender_party: Option<Party>,
    pub receiver_party: Option<Party>,
    pub consignor_party: Option<Party>,
    pub carrier_party: Option<Party>,
    pub freight_forwarder_party: Option<Party>,
    pub shipment: Shipment,
    pub document_reference: Vec<DocumentReference>,
    pub exchange_rate: Vec<ExchangeRate>,
    pub document_distribution: Vec<DocumentDistribution>,
    pub signature: Vec<Signature>,
}

// ─── CertificateOfOrigin ──────────────────────────────────────

/// UBL 2.5 CertificateOfOrigin document.
///
/// Reference: UBL-CertificateOfOrigin-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateOfOrigin {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub description: Vec<Description>,
    pub note: Vec<Note>,
    pub signature: Vec<Signature>,
    pub exporter_party: Option<Party>,
    pub importer_party: Option<Party>,
    pub endorser_party: Vec<Party>,
    pub certificate_of_origin_application: CertificateOfOriginApplication,
    pub issuer_endorsement: Endorsement,
    pub embassy_endorsement: Option<Endorsement>,
    pub insurance_endorsement: Option<Endorsement>,
}

// ─── ForwardingInstructions ──────────────────────────────────────

/// UBL 2.5 ForwardingInstructions document.
///
/// Reference: UBL-ForwardingInstructions-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForwardingInstructions {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub name: Option<Name>,
    pub description: Vec<Description>,
    pub note: Vec<Note>,
    pub consignor_party: Option<Party>,
    pub carrier_party: Option<Party>,
    pub freight_forwarder_party: Option<Party>,
    pub shipment: Shipment,
    pub document_reference: Vec<DocumentReference>,
    pub exchange_rate: Vec<ExchangeRate>,
    pub document_distribution: Vec<DocumentDistribution>,
    pub signature: Vec<Signature>,
}

// ─── TransportationStatus ──────────────────────────────────────

/// UBL 2.5 TransportationStatus document.
///
/// Reference: UBL-TransportationStatus-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportationStatus {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub name: Option<Name>,
    pub description: Vec<Description>,
    pub note: Vec<Note>,
    pub transportation_status_type_code: Option<Code>,
    pub consignment: Vec<Consignment>,
    pub transport_event: Vec<TransportEvent>,
    pub document_reference: Vec<DocumentReference>,
    pub signature: Vec<Signature>,
    pub sender_party: Option<Party>,
    pub receiver_party: Option<Party>,
    pub status_location: Vec<Location>,
    pub status_period: Vec<Period>,
}

// ─── TransportationStatusRequest ──────────────────────────────────────

/// UBL 2.5 TransportationStatusRequest document.
///
/// Reference: UBL-TransportationStatusRequest-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportationStatusRequest {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub name: Option<Name>,
    pub description: Vec<Description>,
    pub note: Vec<Note>,
    pub transportation_status_type_code: Option<Code>,
    pub sender_party: Option<Party>,
    pub receiver_party: Option<Party>,
    pub consignment: Vec<Consignment>,
    pub document_reference: Vec<DocumentReference>,
    pub signature: Vec<Signature>,
    pub requested_status_location: Vec<Location>,
    pub requested_status_period: Vec<Period>,
}

// ─── TransportExecutionPlan ──────────────────────────────────────

/// UBL 2.5 TransportExecutionPlan document.
///
/// Reference: UBL-TransportExecutionPlan-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportExecutionPlan {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub note: Vec<Note>,
    pub transport_service_provider_remarks: Vec<TransportServiceProviderRemarks>,
    pub sender_party: Option<Party>,
    pub receiver_party: Option<Party>,
    pub transport_user_party: Option<Party>,
    pub transport_service_provider_party: Party,
    pub bill_to_party: Option<Party>,
    pub signature: Vec<Signature>,
    pub additional_document_reference: Vec<DocumentReference>,
    pub transport_contract: Option<ExchangeRateContract>,
    pub validity_period: Vec<Period>,
    pub main_transportation_service: Option<TransportationService>,
    pub additional_transportation_service: Vec<TransportationService>,
    pub service_start_time_period: Option<Period>,
    pub service_end_time_period: Option<Period>,
    pub from_location: Option<Location>,
    pub to_location: Option<Location>,
    pub at_location: Option<Location>,
    pub transport_execution_terms: Option<TransportExecutionTerms>,
    pub consignment: Vec<Consignment>,
}

// ─── TransportExecutionPlanRequest ──────────────────────────────────────

/// UBL 2.5 TransportExecutionPlanRequest document.
///
/// Reference: UBL-TransportExecutionPlanRequest-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportExecutionPlanRequest {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub note: Vec<Note>,
    pub transport_service_provider_remarks: Vec<TransportServiceProviderRemarks>,
    pub sender_party: Option<Party>,
    pub receiver_party: Option<Party>,
    pub transport_user_party: Option<Party>,
    pub transport_service_provider_party: Party,
    pub payee_party: Option<Party>,
    pub bill_to_party: Option<Party>,
    pub signature: Vec<Signature>,
    pub additional_document_reference: Vec<DocumentReference>,
    pub transport_contract: Option<ExchangeRateContract>,
    pub main_transportation_service: Option<TransportationService>,
    pub additional_transportation_service: Vec<TransportationService>,
    pub service_start_time_period: Option<Period>,
    pub service_end_time_period: Option<Period>,
    pub from_location: Option<Location>,
    pub to_location: Option<Location>,
    pub at_location: Option<Location>,
    pub transport_execution_terms: Option<TransportExecutionTerms>,
    pub consignment: Vec<Consignment>,
}

// ─── TransportServiceDescription ──────────────────────────────────────

/// UBL 2.5 TransportServiceDescription document.
///
/// Reference: UBL-TransportServiceDescription-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportServiceDescription {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub note: Vec<Note>,
    pub service_name: Option<ServiceName>,
    pub signature: Vec<Signature>,
    pub sender_party: Option<Party>,
    pub receiver_party: Option<Party>,
    pub transport_service_provider_party: Option<Party>,
    pub service_charge_payment_terms: Option<PaymentTermsType>,
    pub validity_period: Option<Period>,
    pub transportation_service: Vec<TransportationService>,
}

// ─── TransportServiceDescriptionRequest ──────────────────────────────────────

/// UBL 2.5 TransportServiceDescriptionRequest document.
///
/// Reference: UBL-TransportServiceDescriptionRequest-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportServiceDescriptionRequest {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    pub issue_time: IssueTime,
    pub note: Vec<Note>,
    pub service_information_preference_code: Option<ServiceInformationPreferenceCode>,
    pub signature: Vec<Signature>,
    pub sender_party: Option<Party>,
    pub receiver_party: Option<Party>,
    pub transport_service_provider_party: Option<Party>,
    pub transportation_service: Vec<TransportationService>,
}

// ─── TransportProgressStatus ──────────────────────────────────────

/// UBL 2.5 TransportProgressStatus document.
///
/// Reference: UBL-TransportProgressStatus-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportProgressStatus {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    pub issue_time: IssueTime,
    pub note: Vec<Note>,
    pub signature: Vec<Signature>,
    pub sender_party: Option<Party>,
    pub receiver_party: Option<Party>,
    pub source_issuer_party: Option<Party>,
    pub transport_means: TransportMeans,
    pub transport_schedule: Vec<TransportSchedule>,
}

// ─── TransportProgressStatusRequest ──────────────────────────────────────

/// UBL 2.5 TransportProgressStatusRequest document.
///
/// Reference: UBL-TransportProgressStatusRequest-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportProgressStatusRequest {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    pub issue_time: IssueTime,
    pub note: Vec<Note>,
    pub signature: Vec<Signature>,
    pub sender_party: Option<Party>,
    pub receiver_party: Option<Party>,
    pub transport_means: TransportMeans,
    pub status_location: Vec<Location>,
}

// ─── GoodsItemItinerary ──────────────────────────────────────

/// UBL 2.5 GoodsItemItinerary document.
///
/// Reference: UBL-GoodsItemItinerary-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodsItemItinerary {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    pub issue_time: IssueTime,
    pub note: Vec<Note>,
    pub transport_execution_plan_reference_id: ID,
    pub signature: Vec<Signature>,
    pub sender_party: Option<Party>,
    pub receiver_party: Option<Party>,
    pub referenced_consignment: Vec<Consignment>,
    pub referenced_transport_equipment: Vec<TransportEquipment>,
    pub referenced_package: Vec<Package>,
    pub referenced_goods_item: Vec<GoodsItem>,
    pub transportation_segment: Vec<TransportationSegment>,
}

// ─── GoodsItemPassport ──────────────────────────────────────

/// UBL 2.5 GoodsItemPassport document.
///
/// Reference: UBL-GoodsItemPassport-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodsItemPassport {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub note: Vec<Note>,
    pub validity_period: Option<Period>,
    pub issuer_party: Option<Party>,
    pub holder_party: Party,
    pub representative_party: Option<Party>,
    pub exporting_guarantor_party: Option<Party>,
    pub importing_guarantor_party: Option<Party>,
    pub exporting_customs_party: Option<Party>,
    pub importing_customs_party: Option<Party>,
    pub shipment: Shipment,
    pub goods_item_passport_counterfoil: Vec<GoodsItemPassportCounterfoil>,
    pub issuer_endorsement: Option<Endorsement>,
    pub additional_document_reference: Vec<DocumentReference>,
    pub document_distribution: Vec<DocumentDistribution>,
    pub signature: Vec<Signature>,
}

// ─── Manifest ──────────────────────────────────────

/// UBL 2.5 Manifest document.
///
/// Reference: UBL-Manifest-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub description: Vec<Description>,
    pub note: Vec<Note>,
    pub sending_logistics_operator_party: Party,
    pub authority_party: Option<Party>,
    pub consignor_party: Option<Party>,
    pub consignee_party: Option<Party>,
    pub shipment: Option<Shipment>,
    pub document_reference: Vec<DocumentReference>,
    pub document_distribution: Vec<DocumentDistribution>,
    pub signature: Vec<Signature>,
}

// ─── CommonTransportationReport ──────────────────────────────────────

/// UBL 2.5 CommonTransportationReport document.
///
/// Reference: UBL-CommonTransportationReport-2.5.xsd
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonTransportationReport {
    pub id: ID,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub name: Option<Name>,
    pub description: Vec<Description>,
    pub note: Vec<Note>,
    pub consignment: Vec<Consignment>,
    pub transport_equipment: Vec<TransportEquipment>,
    pub transport_means: Vec<TransportMeans>,
    pub transportation_service: Vec<TransportationService>,
    pub document_reference: Vec<DocumentReference>,
    pub signature: Vec<Signature>,
}
