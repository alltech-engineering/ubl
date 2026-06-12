// UBL 2.5 CAC Tier 3-4: Transport & Equipment Aggregates
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── TransportEquipment ──────────────────────────────────────────────
// XSD: TransportEquipmentType
// A piece of equipment used to transport goods
// (shipping container, sea container, rail wagon, pallet, trailer, ULD)

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportEquipment {
    pub id: Option<String>,
    pub referenced_consignment_id: Vec<String>,
    pub transport_equipment_type_code: Option<String>,
    pub provider_type_code: Option<String>,
    pub owner_type_code: Option<String>,
    pub size_type_code: Option<String>,
    pub disposition_code: Option<String>,
    pub fullness_indication_code: Option<String>,
    pub refrigeration_on_indicator: Option<bool>,
    pub information: Vec<String>,
    pub returnability_indicator: Option<bool>,
    pub legal_status_indicator: Option<bool>,
    pub air_flow_percent: Option<f64>,
    pub humidity_percent: Option<f64>,
    pub animal_food_approved_indicator: Option<bool>,
    pub human_food_approved_indicator: Option<bool>,
    pub dangerous_goods_approved_indicator: Option<bool>,
    pub refrigerated_indicator: Option<bool>,
    pub characteristics: Option<String>,
    pub damage_remarks: Vec<String>,
    pub description: Vec<String>,
    pub special_transport_requirements: Vec<String>,
    pub gross_weight_measure: Option<f64>,
    pub gross_volume_measure: Option<f64>,
    pub tare_weight_measure: Option<f64>,
    pub tracking_device_code: Option<String>,
    pub power_indicator: Option<bool>,
    pub trace_id: Option<String>,
    pub stowage_position_id: Option<String>,
    // CAC refs resolved at integration:
    // measurement_dimension: Vec<Dimension>
    // transport_equipment_seal: Vec<TransportEquipmentSeal>
    // minimum/maximum_temperature: Option<Temperature>
    // provider/loading_proof/supplier/owner/operating_party: Option<Party>
    // loading/unloading/storage_location: Option<Location>
    // various TransportEvent refs
    // applicable_transport_means: Option<TransportMeans>
    // haulage_trading_terms, hazardous_goods_transit, packaged_transport_handling_unit
    // service/freight_allowance_charge, attached_transport_equipment
    // delivery, pickup, despatch
    // shipment_document_reference, contained_in_transport_equipment
    // package, goods_item, verified_gross_mass, loaded_hazardous_item
}

// ─── TransportHandlingUnit ───────────────────────────────────────────
// XSD: TransportHandlingUnitType
// A piece of equipment used to handle goods

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportHandlingUnit {
    pub id: Option<String>,
    pub transport_handling_unit_type_code: Option<String>,
    pub handling_code: Vec<String>,
    pub handling_instructions: Vec<String>,
    pub hazardous_risk_indicator: Option<bool>,
    pub total_goods_item_quantity: Option<f64>,
    pub total_package_quantity: Option<f64>,
    pub damage_remarks: Vec<String>,
    pub shipping_marks: Vec<String>,
    pub trace_id: Option<String>,
    // CAC refs:
    // handling_unit_despatch_line: Vec<DespatchLine>
    // actual_package: Vec<Package>
    // received_handling_unit_receipt_line: Vec<ReceiptLine>
    // transport_equipment: Vec<TransportEquipment>
    // transport_means: Vec<TransportMeans>
    // hazardous_goods_transit: Vec<HazardousGoodsTransit>
    // measurement_dimension: Vec<Dimension>
    // minimum/maximum_temperature: Option<Temperature>
    // goods_item: Vec<GoodsItem>
    // floor_space_measurement_dimension, pallet_space_measurement_dimension
    // shipment_document_reference: Vec<DocumentReference>
    // status: Vec<Status>
    // customs_declaration: Vec<CustomsDeclaration>
    // referenced_shipment: Vec<Shipment>
    // package: Vec<Package>
    // damage_documentation_attachment
    // energy_consumption_allocation
}

// ─── TransportMeans ──────────────────────────────────────────────────
// XSD: TransportMeansType
// The vehicle, vessel, or other means used to transport goods

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportMeans {
    pub journey_id: Option<String>,
    pub registration_nationality_id: Option<String>,
    pub registration_nationality: Vec<String>,
    pub direction_code: Option<String>,
    pub transport_means_type_code: Option<String>,
    pub trade_service_code: Option<String>,
    // CAC: stowage: Option<Stowage>
    // CAC: air_transport, road_transport, rail_transport, maritime_transport
    // CAC: owner_party: Option<Party>
    // CAC: measurement_dimension: Vec<Dimension>
}

// ─── TransportationService ───────────────────────────────────────────
// XSD: TransportationServiceType
// A transport service (carrier service, shipping line service, etc.)

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportationService {
    pub transport_service_code: String, // 1..1 required
    pub tariff_class_code: Option<String>,
    pub priority: Option<String>,
    pub freight_rate_class_code: Option<String>,
    pub transportation_service_description: Vec<String>,
    pub transportation_service_details_uri: Option<String>,
    pub nomination_date: Option<String>,
    pub nomination_time: Option<String>,
    pub name: Option<String>,
    pub sequence_numeric: Option<f64>,
    // CAC: transport_equipment: Vec<TransportEquipment>
    // CAC: supported/unsupported_transport_equipment
    // CAC: commodity_classification: Vec<CommodityClassification>
    // CAC: supported/unsupported_commodity_classification
    // CAC: total_capacity_dimension: Option<Dimension>
    // CAC: shipment_stage: Vec<ShipmentStage>
    // CAC: transport_event: Vec<TransportEvent>
    // CAC: responsible_transport_service_provider_party: Option<Party>
    // CAC: environmental_emission: Vec<EnvironmentalEmission>
    // CAC: estimated_duration_period: Option<Period>
    // CAC: scheduled_service_frequency: Vec<ServiceFrequency>
}

// ─── Shipment ────────────────────────────────────────────────────────
// XSD: ShipmentType
// An identifiable collection of one or more goods items to be transported

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shipment {
    pub id: Option<String>,
    pub shipping_priority_level_code: Option<String>,
    pub handling_code: Vec<String>,
    pub handling_instructions: Vec<String>,
    pub information: Vec<String>,
    pub gross_weight_measure: Option<f64>,
    pub net_weight_measure: Option<f64>,
    pub net_net_weight_measure: Option<f64>,
    pub gross_volume_measure: Option<f64>,
    pub net_volume_measure: Option<f64>,
    pub total_goods_item_quantity: Option<f64>,
    pub total_transport_handling_unit_quantity: Option<f64>,
    pub insurance_value_amount: Option<f64>,
    pub declared_customs_value_amount: Option<f64>,
    pub declared_for_carriage_value_amount: Option<f64>,
    pub declared_statistics_value_amount: Option<f64>,
    pub free_on_board_value_amount: Option<f64>,
    pub special_instructions: Vec<String>,
    pub delivery_instructions: Vec<String>,
    pub split_consignment_indicator: Option<bool>,
    pub consignment_quantity: Option<f64>,
    // CAC: consignment: Vec<Consignment>
    // CAC: goods_item: Vec<GoodsItem>
    // CAC: shipment_stage: Vec<ShipmentStage>
    // CAC: delivery: Option<Delivery>
    // CAC: transport_handling_unit: Vec<TransportHandlingUnit>
    // CAC: return/origin_address: Option<Address>
    // CAC: first_arrival_port_location, last_exit_port_location: Option<Location>
    // CAC: export_country: Option<Country>
    // CAC: freight_allowance_charge: Vec<AllowanceCharge>
    // CAC: insurance_policy: Vec<InsurancePolicy>
}

// ─── Consignment ─────────────────────────────────────────────────────
// XSD: ConsignmentType
// A separately identifiable collection of goods items to be transported

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Consignment {
    pub id: String, // 1..1 required
    pub carrier_assigned_id: Option<String>,
    pub consignee_assigned_id: Option<String>,
    pub consignor_assigned_id: Option<String>,
    pub freight_forwarder_assigned_id: Option<String>,
    pub broker_assigned_id: Option<String>,
    pub contracted_carrier_assigned_id: Option<String>,
    pub performing_carrier_assigned_id: Option<String>,
    pub summary_description: Vec<String>,
    pub total_invoice_amount: Option<f64>,
    pub declared_customs_value_amount: Option<f64>,
    pub tariff_description: Vec<String>,
    pub tariff_code: Option<String>,
    pub insurance_premium_amount: Option<f64>,
    pub gross_weight_measure: Option<f64>,
    pub net_weight_measure: Option<f64>,
    pub net_net_weight_measure: Option<f64>,
    pub chargeable_weight_measure: Option<f64>,
    pub gross_volume_measure: Option<f64>,
    pub net_volume_measure: Option<f64>,
    pub loading_length_measure: Option<f64>,
    pub remarks: Vec<String>,
    pub hazardous_risk_indicator: Option<bool>,
    pub animal_food_indicator: Option<bool>,
    pub human_food_indicator: Option<bool>,
    pub livestock_indicator: Option<bool>,
    pub bulk_cargo_indicator: Option<bool>,
    pub containerized_indicator: Option<bool>,
    pub general_cargo_indicator: Option<bool>,
    pub special_security_indicator: Option<bool>,
    pub third_party_payer_indicator: Option<bool>,
    pub carrier_service_instructions: Vec<String>,
    pub customs_clearance_service_instructions: Vec<String>,
    pub forwarder_service_instructions: Vec<String>,
    pub special_service_instructions: Vec<String>,
    pub sequence_id: Option<String>,
    pub shipping_priority_level_code: Option<String>,
    pub handling_code: Vec<String>,
    pub handling_instructions: Vec<String>,
    pub information: Vec<String>,
    pub total_goods_item_quantity: Option<f64>,
    pub total_transport_handling_unit_quantity: Option<f64>,
    pub insurance_value_amount: Option<f64>,
    pub declared_for_carriage_value_amount: Option<f64>,
    pub declared_statistics_value_amount: Option<f64>,
    pub free_on_board_value_amount: Option<f64>,
    pub special_instructions: Vec<String>,
    pub split_consignment_indicator: Option<bool>,
    pub delivery_instructions: Vec<String>,
    pub consignment_quantity: Option<f64>,
    pub consolidatable_indicator: Option<bool>,
    pub haulage_instructions: Vec<String>,
    pub loading_sequence_id: Option<String>,
    pub child_consignment_quantity: Option<f64>,
    pub total_packages_quantity: Option<f64>,
    // CAC: consolidated_shipment: Vec<Shipment>
    // CAC: customs_declaration: Vec<CustomsDeclaration>
    // CAC: requested/planned/actual_pickup/delivery_transport_event
    // CAC: status: Vec<Status>
    // CAC: child_consignment: Vec<Consignment>
    // CAC: multiple Party references (consignee, exporter, consignor, importer, etc.)
    // CAC: country references (original_departure, final_destination, transit)
    // CAC: transport_contract, transport_event
    // CAC: transportation_service references
    // CAC: delivery/payment/collect/disbursement/prepaid_terms
    // CAC: freight/extra_allowance_charge
    // CAC: shipment_stage references
    // CAC: transport_handling_unit
    // CAC: multiple Location references
    // CAC: document_reference: Vec<DocumentReference>
    // CAC: environmental_emission: Vec<EnvironmentalEmission>
    // CAC: insurance_policy: Vec<InsurancePolicy>
}

// ─── ShipmentStage ───────────────────────────────────────────────────
// XSD: ShipmentStageType
// One stage in a transport movement

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShipmentStage {
    pub id: Option<String>,
    pub shipment_stage_type_code: Option<String>,
    pub shipment_stage_type: Vec<String>,
    pub transport_mode_code: Option<String>,
    pub transport_means_type_code: Option<String>,
    pub transit_direction_code: Option<String>,
    pub pre_carriage_indicator: Option<bool>,
    pub on_carriage_indicator: Option<bool>,
    pub cabotage_indicator: Option<bool>,
    pub hazardous_risk_indicator: Option<bool>,
    pub estimated_delivery_date: Option<String>,
    pub estimated_delivery_time: Option<String>,
    pub required_delivery_date: Option<String>,
    pub required_delivery_time: Option<String>,
    pub loading_sequence_id: Option<String>,
    pub successive_sequence_id: Option<String>,
    pub instructions: Vec<String>,
    pub demurrage_instructions: Vec<String>,
    pub crew_quantity: Option<f64>,
    pub passenger_quantity: Option<f64>,
    // CAC: transit_period: Option<Period>
    // CAC: carrier_party: Vec<Party>
    // CAC: transport_means: Option<TransportMeans>
    // CAC: multiple Location references
    // CAC: multiple TransportEvent references
    // CAC: various Party references
    // CAC: estimated_transit_period: Option<Period>
    // CAC: freight_allowance_charge: Vec<AllowanceCharge>
    // CAC: freight_charge_location: Option<Location>
    // CAC: person references (passenger, driver, crew, master, etc.)
    // CAC: maritime references (port_call, waste, ballast, ISPS, health)
    // CAC: fuel_consumption: Vec<FuelConsumption>
}

// ─── TransportEvent ──────────────────────────────────────────────────
// XSD: TransportEventType
// An event in the transport of goods

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportEvent {
    pub identification_id: Option<String>,
    pub occurrence_date: Option<String>,
    pub occurrence_time: Option<String>,
    pub transport_event_type_code: Option<String>,
    pub description: Vec<String>,
    pub completion_indicator: Option<bool>,
    // CAC: reported_shipment: Option<Shipment>
    // CAC: current_status: Vec<Status>
    // CAC: responsible_party: Option<Party>
    // CAC: contact: Vec<Contact>
    // CAC: location: Option<Location>
    // CAC: signature: Option<Signature>
    // CAC: period: Vec<Period>
}
