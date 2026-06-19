#[derive(Debug, Deserialize, Serialize)]
pub struct ShipmentStage {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "ShipmentStageTypeCode")]
    pub shipment_stage_type_code: Option<cct::Code>,
    #[serde(default, rename = "ShipmentStageType")]
    pub shipment_stage_type: Vec<cct::Text>,
    #[serde(default, rename = "TransportModeCode")]
    pub transport_mode_code: Option<cct::Code>,
    #[serde(default, rename = "TransportMeansTypeCode")]
    pub transport_means_type_code: Option<cct::Code>,
    #[serde(default, rename = "TransitDirectionCode")]
    pub transit_direction_code: Option<cct::Code>,
    #[serde(default, rename = "PreCarriageIndicator")]
    pub pre_carriage_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "OnCarriageIndicator")]
    pub on_carriage_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "CabotageIndicator")]
    pub cabotage_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "EstimatedDeliveryDate")]
    pub estimated_delivery_date: Option<udt::DateTime>,
    #[serde(default, rename = "EstimatedDeliveryTime")]
    pub estimated_delivery_time: Option<udt::DateTime>,
    #[serde(default, rename = "RequiredDeliveryDate")]
    pub required_delivery_date: Option<udt::DateTime>,
    #[serde(default, rename = "RequiredDeliveryTime")]
    pub required_delivery_time: Option<udt::DateTime>,
    #[serde(default, rename = "LoadingSequenceID")]
    pub loading_sequence_id: Option<cct::Identifier>,
    #[serde(default, rename = "SuccessiveSequenceID")]
    pub successive_sequence_id: Option<cct::Identifier>,
    #[serde(default, rename = "Instructions")]
    pub instructions: Vec<cct::Text>,
    #[serde(default, rename = "DemurrageInstructions")]
    pub demurrage_instructions: Vec<cct::Text>,
    #[serde(default, rename = "CrewQuantity")]
    pub crew_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "PassengerQuantity")]
    pub passenger_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "TransitPeriod")]
    pub transit_period: Option<crate::Period>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: Vec<crate::Party>,
    #[serde(default, rename = "TransportMeans")]
    pub transport_means: Option<crate::TransportMeans>,
    #[serde(default, rename = "LoadingPortLocation")]
    pub loading_port_location: Option<crate::Location>,
    #[serde(default, rename = "UnloadingPortLocation")]
    pub unloading_port_location: Option<crate::Location>,
    #[serde(default, rename = "TransshipPortLocation")]
    pub transship_port_location: Option<crate::Location>,
    #[serde(default, rename = "LoadingTransportEvent")]
    pub loading_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "ExaminationTransportEvent")]
    pub examination_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "AvailabilityTransportEvent")]
    pub availability_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "ExportationTransportEvent")]
    pub exportation_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "DischargeTransportEvent")]
    pub discharge_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "WarehousingTransportEvent")]
    pub warehousing_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "TakeoverTransportEvent")]
    pub takeover_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "OptionalTakeoverTransportEvent")]
    pub optional_takeover_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "DropoffTransportEvent")]
    pub dropoff_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "ActualPickupTransportEvent")]
    pub actual_pickup_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "DeliveryTransportEvent")]
    pub delivery_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "ReceiptTransportEvent")]
    pub receipt_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "StorageTransportEvent")]
    pub storage_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "AcceptanceTransportEvent")]
    pub acceptance_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "TerminalOperatorParty")]
    pub terminal_operator_party: Option<crate::Party>,
    #[serde(default, rename = "CustomsAgentParty")]
    pub customs_agent_party: Option<crate::Party>,
    #[serde(default, rename = "EstimatedTransitPeriod")]
    pub estimated_transit_period: Option<crate::Period>,
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: Vec<crate::AllowanceCharge>,
    #[serde(default, rename = "FreightChargeLocation")]
    pub freight_charge_location: Option<crate::Location>,
    #[serde(default, rename = "DetentionTransportEvent")]
    pub detention_transport_event: Vec<crate::TransportEvent>,
    #[serde(default, rename = "RequestedDepartureTransportEvent")]
    pub requested_departure_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "RequestedArrivalTransportEvent")]
    pub requested_arrival_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "RequestedWaypointTransportEvent")]
    pub requested_waypoint_transport_event: Vec<crate::TransportEvent>,
    #[serde(default, rename = "PlannedDepartureTransportEvent")]
    pub planned_departure_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "PlannedArrivalTransportEvent")]
    pub planned_arrival_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "PlannedWaypointTransportEvent")]
    pub planned_waypoint_transport_event: Vec<crate::TransportEvent>,
    #[serde(default, rename = "ActualDepartureTransportEvent")]
    pub actual_departure_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "ActualWaypointTransportEvent")]
    pub actual_waypoint_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "ActualArrivalTransportEvent")]
    pub actual_arrival_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: Vec<crate::TransportEvent>,
    #[serde(default, rename = "EstimatedDepartureTransportEvent")]
    pub estimated_departure_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "EstimatedArrivalTransportEvent")]
    pub estimated_arrival_transport_event: Option<crate::TransportEvent>,
    #[serde(default, rename = "PassengerPerson")]
    pub passenger_person: Vec<crate::Person>,
    #[serde(default, rename = "DriverPerson")]
    pub driver_person: Vec<crate::Person>,
    #[serde(default, rename = "ReportingPerson")]
    pub reporting_person: Option<crate::Person>,
    #[serde(default, rename = "CrewMemberPerson")]
    pub crew_member_person: Vec<crate::Person>,
    #[serde(default, rename = "SecurityOfficerPerson")]
    pub security_officer_person: Option<crate::Person>,
    #[serde(default, rename = "MasterPerson")]
    pub master_person: Option<crate::Person>,
    #[serde(default, rename = "ShipsSurgeonPerson")]
    pub ships_surgeon_person: Option<crate::Person>,
    #[serde(default, rename = "DestinationPortCall")]
    pub destination_port_call: Option<crate::PortCall>,
    #[serde(default, rename = "ShipStoreArticle")]
    pub ship_store_article: Vec<crate::ShipStoreArticle>,
    #[serde(default, rename = "CrewPersonEffect")]
    pub crew_person_effect: Vec<crate::CrewPersonEffect>,
    #[serde(default, rename = "MaritimeWaste")]
    pub maritime_waste: Vec<crate::MaritimeWaste>,
    #[serde(default, rename = "BallastWaterSummary")]
    pub ballast_water_summary: Option<crate::BallastWaterSummary>,
    #[serde(default, rename = "ISPSRequirements")]
    pub isps_requirements: Option<crate::IspsRequirements>,
    #[serde(default, rename = "MaritimeHealthDeclaration")]
    pub maritime_health_declaration: Option<crate::MaritimeHealthDeclaration>,
    #[serde(default, rename = "FuelConsumption")]
    pub fuel_consumption: Vec<crate::FuelConsumption>,
}
