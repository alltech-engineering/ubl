// UBL Transport aggregates — equipment, handling units, means, and services.

use crate::cac::party::Party;
use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportEquipment {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub transport_equipment_type_code: Option<TransportEquipmentTypeCode>,
    #[serde(default)]
    pub description: Vec<Description>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportHandlingUnit {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub transport_handling_unit_type_code: Option<TransportHandlingUnitTypeCode>,
    #[serde(default)]
    pub handling_code: Option<HandlingCode>,
    #[serde(default)]
    pub handling_instructions: Vec<HandlingInstructions>,
    #[serde(default)]
    pub hazardous_risk_indicator: Option<HazardousRiskIndicator>,
    #[serde(default)]
    pub total_goods_item_quantity: Option<TotalGoodsItemQuantity>,
    #[serde(default)]
    pub total_package_quantity: Option<TotalPackagesQuantity>,
    #[serde(default)]
    pub damage_remarks: Vec<DamageRemarks>,
    #[serde(default)]
    pub shipping_marks: Vec<ShippingMarks>,
    #[serde(default)]
    pub temperature: Vec<Temperature>,
    #[serde(default)]
    pub goods_item: Vec<GoodsItem>,
    #[serde(default)]
    pub transport_equipment: Vec<TransportEquipment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportMeans {
    #[serde(default)]
    pub journey_id: Option<ID>,
    #[serde(default)]
    pub registration_nationality_id: Option<RegistrationNationalityID>,
    #[serde(default)]
    pub registration_nationality: Vec<Text>,
    #[serde(default)]
    pub direction_code: Option<DirectionCode>,
    #[serde(default)]
    pub transport_means_type_code: Option<TransportMeansTypeCode>,
    #[serde(default)]
    pub trade_service_code: Option<TradeServiceCode>,
    #[serde(default)]
    pub stowage_value_text: Vec<Text>,
    #[serde(default)]
    pub stowage_value_quantity: Option<Quantity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportationService {
    pub transport_service_code: TransportServiceCode,
    #[serde(default)]
    pub tariff_class_code: Option<TariffClassCode>,
    #[serde(default)]
    pub priority: Option<Text>,
    #[serde(default)]
    pub freight_rate_class_code: Option<Code>,
    #[serde(default)]
    pub transportation_service_description: Vec<Text>,
    #[serde(default)]
    pub transportation_service_details_uri: Option<Text>,
    #[serde(default)]
    pub nomination_date: Option<Date>,
    #[serde(default)]
    pub nomination_time: Option<Time>,
    #[serde(default)]
    pub name: Option<Name>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Temperature {
    #[serde(default)]
    pub attribute_id: Option<AttributeID>,
    pub measure: TemperatureMeasure,
    #[serde(default)]
    pub description: Vec<Description>,
}

// ─── Consignment ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consignment {
    pub id: ID,
    #[serde(default)]
    pub carrier_assigned_id: Option<CarrierAssignedID>,
    #[serde(default)]
    pub consignee_assigned_id: Option<ConsigneeAssignedID>,
    #[serde(default)]
    pub consignor_assigned_id: Option<ConsignorAssignedID>,
    #[serde(default)]
    pub summary_description: Vec<Description>,
    #[serde(default)]
    pub total_invoice_amount: Option<Amount>,
    #[serde(default)]
    pub declared_customs_value_amount: Option<Amount>,
    #[serde(default)]
    pub gross_weight_measure: Option<Measure>,
    #[serde(default)]
    pub net_weight_measure: Option<Measure>,
    #[serde(default)]
    pub gross_volume_measure: Option<Measure>,
    #[serde(default)]
    pub net_volume_measure: Option<Measure>,
    #[serde(default)]
    pub handling_code: Vec<Code>,
    #[serde(default)]
    pub handling_instructions: Vec<HandlingInstructions>,
    #[serde(default)]
    pub total_goods_item_quantity: Option<Quantity>,
    #[serde(default)]
    pub total_transport_handling_unit_quantity: Option<Quantity>,
    #[serde(default)]
    pub transport_handling_unit: Vec<TransportHandlingUnit>,
}

// ─── TransportEvent ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportEvent {
    #[serde(default)]
    pub identification_id: Option<ID>,
    #[serde(default)]
    pub occurrence_date: Option<Date>,
    #[serde(default)]
    pub occurrence_time: Option<Time>,
    #[serde(default)]
    pub transport_event_type_code: Option<Code>,
    #[serde(default)]
    pub description: Vec<Description>,
    #[serde(default)]
    pub completion_indicator: Option<Indicator>,
    #[serde(default)]
    pub location: Option<Location>,
    #[serde(default)]
    pub period: Vec<Period>,
}

// ─── TransportSchedule ───────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportSchedule {
    pub sequence_numeric: Numeric,
    #[serde(default)]
    pub reference_date: Option<Date>,
    #[serde(default)]
    pub reference_time: Option<Time>,
    #[serde(default)]
    pub reliability_percent: Option<Percent>,
    #[serde(default)]
    pub remarks: Vec<Text>,
    pub status_location: Location,
}

// ─── TransportationSegment ───────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportationSegment {
    pub sequence_numeric: Numeric,
    #[serde(default)]
    pub transport_execution_plan_reference_id: Option<ID>,
    pub transportation_service: TransportationService,
    pub transport_service_provider_party: Party,
    #[serde(default)]
    pub referenced_consignment: Option<Consignment>,
}

// ─── TransportExecutionTerms ─────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportExecutionTerms {
    #[serde(default)]
    pub transport_user_special_terms: Vec<Text>,
    #[serde(default)]
    pub transport_service_provider_special_terms: Vec<Text>,
    #[serde(default)]
    pub change_conditions: Vec<Text>,
}

// ─── Endorsement ─────────────────────────────────────────────────────
// Used by: IssuerEndorsement, EmbassyEndorsement, InsuranceEndorsement

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Endorsement {
    pub id: ID,
    #[serde(default)]
    pub endorsement_qualifier: Option<Text>,
    pub endorser_party: Party,
    #[serde(default)]
    pub signature: Vec<Signature>,
}

// ─── CertificateOfOriginApplication ──────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificateOfOriginApplication {
    pub reference_id: ID,
    pub certificate_type: Code,
    #[serde(default)]
    pub application_status_code: Option<Code>,
    pub original_job_id: ID,
    #[serde(default)]
    pub previous_job_id: Option<ID>,
    #[serde(default)]
    pub remarks: Vec<Text>,
    pub shipment: Shipment,
    #[serde(default)]
    pub endorser_party: Vec<Party>,
    pub preparation_party: Party,
    pub issuer_party: Party,
    #[serde(default)]
    pub exporter_party: Option<Party>,
    #[serde(default)]
    pub importer_party: Option<Party>,
    pub issuing_country: Country,
    #[serde(default)]
    pub document_distribution: Vec<DocumentDistribution>,
    #[serde(default)]
    pub signature: Vec<Signature>,
}

// ─── GoodsItemPassportCounterfoil ────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodsItemPassportCounterfoil {
    pub id: ID,
    #[serde(default)]
    pub goods_item_passport_id: Option<ID>,
    #[serde(default)]
    pub final_reexportation_date: Option<Date>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub goods_item: Option<GoodsItem>,
    #[serde(default)]
    pub exportation_document_reference: Vec<DocumentReference>,
    #[serde(default)]
    pub importation_document_reference: Vec<DocumentReference>,
}

// ─── Package ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Package {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub quantity: Option<Quantity>,
    #[serde(default)]
    pub returnable_material_indicator: Option<Indicator>,
    #[serde(default)]
    pub package_level_code: Option<Code>,
    #[serde(default)]
    pub packaging_type_code: Option<Code>,
    #[serde(default)]
    pub packaging_type: Vec<Text>,
    #[serde(default)]
    pub packing_material: Vec<Text>,
}

// ─── Status ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    #[serde(default)]
    pub condition_code: Option<Code>,
    #[serde(default)]
    pub reference_date: Option<Date>,
    #[serde(default)]
    pub reference_time: Option<Time>,
    #[serde(default)]
    pub description: Vec<Description>,
    #[serde(default)]
    pub status_reason_code: Option<Code>,
    #[serde(default)]
    pub status_reason: Vec<Text>,
    #[serde(default)]
    pub sequence_id: Option<ID>,
    #[serde(default)]
    pub text: Vec<Text>,
    #[serde(default)]
    pub indication_indicator: Option<Indicator>,
    #[serde(default)]
    pub percent: Option<Percent>,
}

// ─── EnvironmentalEmission ───────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalEmission {
    pub environmental_emission_type_code: Code,
    pub value_measure: Measure,
    #[serde(default)]
    pub description: Vec<Description>,
}

// ─── NotificationRequirement ─────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationRequirement {
    pub notification_type_code: Code,
    #[serde(default)]
    pub post_event_notification_duration_measure: Option<Measure>,
    #[serde(default)]
    pub pre_event_notification_duration_measure: Option<Measure>,
}

// ─── Forward declarations for cross-module types ─────────────────────
use crate::cac::address::Country;
use crate::cac::delivery::GoodsItem;
use crate::cac::delivery::Shipment;
use crate::cac::document::{DocumentDistribution, DocumentReference, Signature};
use crate::cac::party::Location;
use crate::cac::period::Period;
