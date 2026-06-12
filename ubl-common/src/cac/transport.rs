// UBL Transport aggregates — equipment, handling units, means, and services.

use serde::{Deserialize, Serialize};
use crate::cbc::*;
use crate::cac::party::Party;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportEquipment {
    pub id: Option<ID>,
    pub transport_equipment_type_code: Option<TransportEquipmentTypeCode>,
    pub description: Vec<Description>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportHandlingUnit {
    pub id: Option<ID>,
    pub transport_handling_unit_type_code: Option<TransportHandlingUnitTypeCode>,
    pub handling_code: Option<HandlingCode>,
    pub handling_instructions: Vec<HandlingInstructions>,
    pub hazardous_risk_indicator: Option<HazardousRiskIndicator>,
    pub total_goods_item_quantity: Option<TotalGoodsItemQuantity>,
    pub total_package_quantity: Option<TotalPackagesQuantity>,
    pub damage_remarks: Vec<DamageRemarks>,
    pub shipping_marks: Vec<ShippingMarks>,
    pub temperature: Vec<Temperature>,
    pub goods_item: Vec<GoodsItem>,
    pub transport_equipment: Vec<TransportEquipment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportMeans {
    pub journey_id: Option<ID>,
    pub registration_nationality_id: Option<RegistrationNationalityID>,
    pub registration_nationality: Vec<Text>,
    pub direction_code: Option<DirectionCode>,
    pub transport_means_type_code: Option<TransportMeansTypeCode>,
    pub trade_service_code: Option<TradeServiceCode>,
    pub stowage_value_text: Vec<Text>,
    pub stowage_value_quantity: Option<Quantity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportationService {
    pub transport_service_code: TransportServiceCode,
    pub tariff_class_code: Option<TariffClassCode>,
    pub priority: Option<Text>,
    pub freight_rate_class_code: Option<Code>,
    pub transportation_service_description: Vec<Text>,
    pub transportation_service_details_uri: Option<Text>,
    pub nomination_date: Option<Date>,
    pub nomination_time: Option<Time>,
    pub name: Option<Name>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Temperature {
    pub attribute_id: Option<AttributeID>,
    pub measure: TemperatureMeasure,
    pub description: Vec<Description>,
}

// ─── Consignment ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consignment {
    pub id: ID,
    pub carrier_assigned_id: Option<CarrierAssignedID>,
    pub consignee_assigned_id: Option<ConsigneeAssignedID>,
    pub consignor_assigned_id: Option<ConsignorAssignedID>,
    pub summary_description: Vec<Description>,
    pub total_invoice_amount: Option<Amount>,
    pub declared_customs_value_amount: Option<Amount>,
    pub gross_weight_measure: Option<Measure>,
    pub net_weight_measure: Option<Measure>,
    pub gross_volume_measure: Option<Measure>,
    pub net_volume_measure: Option<Measure>,
    pub handling_code: Vec<Code>,
    pub handling_instructions: Vec<HandlingInstructions>,
    pub total_goods_item_quantity: Option<Quantity>,
    pub total_transport_handling_unit_quantity: Option<Quantity>,
    pub transport_handling_unit: Vec<TransportHandlingUnit>,
}

// ─── TransportEvent ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportEvent {
    pub identification_id: Option<ID>,
    pub occurrence_date: Option<Date>,
    pub occurrence_time: Option<Time>,
    pub transport_event_type_code: Option<Code>,
    pub description: Vec<Description>,
    pub completion_indicator: Option<Indicator>,
    pub location: Option<Location>,
    pub period: Vec<Period>,
}

// ─── TransportSchedule ───────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportSchedule {
    pub sequence_numeric: Numeric,
    pub reference_date: Option<Date>,
    pub reference_time: Option<Time>,
    pub reliability_percent: Option<Percent>,
    pub remarks: Vec<Text>,
    pub status_location: Location,
}

// ─── TransportationSegment ───────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportationSegment {
    pub sequence_numeric: Numeric,
    pub transport_execution_plan_reference_id: Option<ID>,
    pub transportation_service: TransportationService,
    pub transport_service_provider_party: Party,
    pub referenced_consignment: Option<Consignment>,
}

// ─── TransportExecutionTerms ─────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportExecutionTerms {
    pub transport_user_special_terms: Vec<Text>,
    pub transport_service_provider_special_terms: Vec<Text>,
    pub change_conditions: Vec<Text>,
}

// ─── Endorsement ─────────────────────────────────────────────────────
// Used by: IssuerEndorsement, EmbassyEndorsement, InsuranceEndorsement

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Endorsement {
    pub id: ID,
    pub endorsement_qualifier: Option<Text>,
    pub endorser_party: Party,
    pub signature: Vec<Signature>,
}

// ─── CertificateOfOriginApplication ──────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateOfOriginApplication {
    pub reference_id: ID,
    pub certificate_type: Code,
    pub application_status_code: Option<Code>,
    pub original_job_id: ID,
    pub previous_job_id: Option<ID>,
    pub remarks: Vec<Text>,
    pub shipment: Shipment,
    pub endorser_party: Vec<Party>,
    pub preparation_party: Party,
    pub issuer_party: Party,
    pub exporter_party: Option<Party>,
    pub importer_party: Option<Party>,
    pub issuing_country: Country,
    pub document_distribution: Vec<DocumentDistribution>,
    pub signature: Vec<Signature>,
}

// ─── GoodsItemPassportCounterfoil ────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodsItemPassportCounterfoil {
    pub id: ID,
    pub goods_item_passport_id: Option<ID>,
    pub final_reexportation_date: Option<Date>,
    pub note: Vec<Note>,
    pub goods_item: Option<GoodsItem>,
    pub exportation_document_reference: Vec<DocumentReference>,
    pub importation_document_reference: Vec<DocumentReference>,
}

// ─── Package ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub id: Option<ID>,
    pub quantity: Option<Quantity>,
    pub returnable_material_indicator: Option<Indicator>,
    pub package_level_code: Option<Code>,
    pub packaging_type_code: Option<Code>,
    pub packaging_type: Vec<Text>,
    pub packing_material: Vec<Text>,
}

// ─── Status ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub condition_code: Option<Code>,
    pub reference_date: Option<Date>,
    pub reference_time: Option<Time>,
    pub description: Vec<Description>,
    pub status_reason_code: Option<Code>,
    pub status_reason: Vec<Text>,
    pub sequence_id: Option<ID>,
    pub text: Vec<Text>,
    pub indication_indicator: Option<Indicator>,
    pub percent: Option<Percent>,
}

// ─── EnvironmentalEmission ───────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalEmission {
    pub environmental_emission_type_code: Code,
    pub value_measure: Measure,
    pub description: Vec<Description>,
}

// ─── NotificationRequirement ─────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationRequirement {
    pub notification_type_code: Code,
    pub post_event_notification_duration_measure: Option<Measure>,
    pub pre_event_notification_duration_measure: Option<Measure>,
}

// ─── Forward declarations for cross-module types ─────────────────────
use crate::cac::delivery::GoodsItem;
use crate::cac::delivery::Shipment;
use crate::cac::document::{DocumentReference, DocumentDistribution, Signature};
use crate::cac::party::Location;
use crate::cac::period::Period;
use crate::cac::address::Country;
