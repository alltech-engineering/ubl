#[derive(Debug, Deserialize, Serialize)]
/// A class to describe one stage of movement in a transport of goods.
///
/// UBL Dictionary Entry Name: `Shipment Stage. Details`
///
/// Generated from XSD type `ShipmentStageType`.
pub struct ShipmentStage {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this shipment stage.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The type of shipment stage, expressed as a code.
    #[serde(default, rename = "ShipmentStageTypeCode")]
    pub shipment_stage_type_code: Option<cct::Code>,
/// The type of shipment stage, expressed as text.
    #[serde(default, rename = "ShipmentStageType")]
    pub shipment_stage_type: Vec<cct::Text>,
/// A code signifying the method of transport used for this shipment stage.
    #[serde(default, rename = "TransportModeCode")]
    pub transport_mode_code: Option<cct::Code>,
/// A code signifying the kind of transport means (truck, vessel, etc.) used for this shipment stage.
    #[serde(default, rename = "TransportMeansTypeCode")]
    pub transport_means_type_code: Option<cct::Code>,
/// A code signifying the direction of transit in this shipment stage.
    #[serde(default, rename = "TransitDirectionCode")]
    pub transit_direction_code: Option<cct::Code>,
/// An indicator that this stage takes place before the main carriage of the shipment (true) or not
/// (false).
    #[serde(default, rename = "PreCarriageIndicator")]
    pub pre_carriage_indicator: Option<udt::Indicator>,
/// An indicator that this stage takes place after the main carriage of the shipment (true) or not
/// (false).
    #[serde(default, rename = "OnCarriageIndicator")]
    pub on_carriage_indicator: Option<udt::Indicator>,
/// An indicator that cabotage applies for this shipment stage (true) or not (false).
    #[serde(default, rename = "CabotageIndicator")]
    pub cabotage_indicator: Option<udt::Indicator>,
/// An indicator that the transported goods in this shipment stage are subject to an international
/// regulation concerning the carriage of dangerous goods (true) or not (false).
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<udt::Indicator>,
/// The estimated date of delivery in this shipment stage.
    #[serde(default, rename = "EstimatedDeliveryDate")]
    pub estimated_delivery_date: Option<udt::DateTime>,
/// The estimated time of delivery in this shipment stage.
    #[serde(default, rename = "EstimatedDeliveryTime")]
    pub estimated_delivery_time: Option<udt::DateTime>,
/// The delivery date required by the buyer in this shipment stage.
    #[serde(default, rename = "RequiredDeliveryDate")]
    pub required_delivery_date: Option<udt::DateTime>,
/// The delivery time required by the buyer in this shipment stage.
    #[serde(default, rename = "RequiredDeliveryTime")]
    pub required_delivery_time: Option<udt::DateTime>,
/// An identifier for the loading sequence (of consignments) associated with this shipment stage.
    #[serde(default, rename = "LoadingSequenceID")]
    pub loading_sequence_id: Option<cct::Identifier>,
/// Identifies the successive loading sequence (of consignments) associated with a shipment stage.
    #[serde(default, rename = "SuccessiveSequenceID")]
    pub successive_sequence_id: Option<cct::Identifier>,
/// Text of instructions applicable to a shipment stage.
    #[serde(default, rename = "Instructions")]
    pub instructions: Vec<cct::Text>,
/// Text of instructions relating to demurrage (the case in which a vessel is prevented from loading or
/// discharging cargo within the stipulated laytime).
    #[serde(default, rename = "DemurrageInstructions")]
    pub demurrage_instructions: Vec<cct::Text>,
/// The total number of crew aboard a transport means.
    #[serde(default, rename = "CrewQuantity")]
    pub crew_quantity: Option<cct::Quantity>,
/// The total number of passengers aboard a transport means.
    #[serde(default, rename = "PassengerQuantity")]
    pub passenger_quantity: Option<cct::Quantity>,
/// The period during which this shipment stage actually took place.
    #[serde(default, rename = "TransitPeriod")]
    pub transit_period: Option<crate::Period>,
/// The Party who provides the transport of goods between named points.
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: Vec<crate::Party>,
/// The means of transport used in this shipment stage.
    #[serde(default, rename = "TransportMeans")]
    pub transport_means: Option<crate::TransportMeans>,
/// The location of loading for a shipment stage.
    #[serde(default, rename = "LoadingPortLocation")]
    pub loading_port_location: Option<crate::Location>,
/// The location of unloading for a shipment stage.
    #[serde(default, rename = "UnloadingPortLocation")]
    pub unloading_port_location: Option<crate::Location>,
/// The location of transshipment relating to a shipment stage.
    #[serde(default, rename = "TransshipPortLocation")]
    pub transship_port_location: Option<crate::Location>,
/// The loading of goods in this shipment stage.
    #[serde(default, rename = "LoadingTransportEvent")]
    pub loading_transport_event: Option<crate::TransportEvent>,
/// The examination of shipments in this shipment stage.
    #[serde(default, rename = "ExaminationTransportEvent")]
    pub examination_transport_event: Option<crate::TransportEvent>,
/// The making available of shipments in this shipment stage.
    #[serde(default, rename = "AvailabilityTransportEvent")]
    pub availability_transport_event: Option<crate::TransportEvent>,
/// The export event associated with this shipment stage.
    #[serde(default, rename = "ExportationTransportEvent")]
    pub exportation_transport_event: Option<crate::TransportEvent>,
/// The discharge event associated with this shipment stage.
    #[serde(default, rename = "DischargeTransportEvent")]
    pub discharge_transport_event: Option<crate::TransportEvent>,
/// The warehousing event associated with this shipment stage.
    #[serde(default, rename = "WarehousingTransportEvent")]
    pub warehousing_transport_event: Option<crate::TransportEvent>,
/// The receiver's takeover of the goods in this shipment stage.
    #[serde(default, rename = "TakeoverTransportEvent")]
    pub takeover_transport_event: Option<crate::TransportEvent>,
/// The optional takeover of the goods in this shipment stage.
    #[serde(default, rename = "OptionalTakeoverTransportEvent")]
    pub optional_takeover_transport_event: Option<crate::TransportEvent>,
/// The dropping off of goods in this shipment stage.
    #[serde(default, rename = "DropoffTransportEvent")]
    pub dropoff_transport_event: Option<crate::TransportEvent>,
/// The pickup of goods in this shipment stage.
    #[serde(default, rename = "ActualPickupTransportEvent")]
    pub actual_pickup_transport_event: Option<crate::TransportEvent>,
/// The delivery of goods in this shipment stage.
    #[serde(default, rename = "DeliveryTransportEvent")]
    pub delivery_transport_event: Option<crate::TransportEvent>,
/// The receipt of goods in this shipment stage.
    #[serde(default, rename = "ReceiptTransportEvent")]
    pub receipt_transport_event: Option<crate::TransportEvent>,
/// The storage of goods in this shipment stage.
    #[serde(default, rename = "StorageTransportEvent")]
    pub storage_transport_event: Option<crate::TransportEvent>,
/// The acceptance of goods in this shipment stage.
    #[serde(default, rename = "AcceptanceTransportEvent")]
    pub acceptance_transport_event: Option<crate::TransportEvent>,
/// A terminal operator associated with this shipment stage.
    #[serde(default, rename = "TerminalOperatorParty")]
    pub terminal_operator_party: Option<crate::Party>,
/// The Customs Agent who is associated with this Shipment Stage.
    #[serde(default, rename = "CustomsAgentParty")]
    pub customs_agent_party: Option<crate::Party>,
/// The estimated transit period of this shipment stage.
    #[serde(default, rename = "EstimatedTransitPeriod")]
    pub estimated_transit_period: Option<crate::Period>,
/// A freight allowance charge for this shipment stage.
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: Vec<crate::AllowanceCharge>,
/// The location associated with a freight charge related to this shipment stage.
    #[serde(default, rename = "FreightChargeLocation")]
    pub freight_charge_location: Option<crate::Location>,
/// The detention of a transport means during loading and unloading operations.
    #[serde(default, rename = "DetentionTransportEvent")]
    pub detention_transport_event: Vec<crate::TransportEvent>,
/// The departure requested by the party requesting a transportation service.
    #[serde(default, rename = "RequestedDepartureTransportEvent")]
    pub requested_departure_transport_event: Option<crate::TransportEvent>,
/// The arrival requested by the party requesting a transportation service.
    #[serde(default, rename = "RequestedArrivalTransportEvent")]
    pub requested_arrival_transport_event: Option<crate::TransportEvent>,
/// A waypoint requested by the party requesting a transportation service.
    #[serde(default, rename = "RequestedWaypointTransportEvent")]
    pub requested_waypoint_transport_event: Vec<crate::TransportEvent>,
/// The departure planned by the party providing a transportation service.
    #[serde(default, rename = "PlannedDepartureTransportEvent")]
    pub planned_departure_transport_event: Option<crate::TransportEvent>,
/// The arrival planned by the party providing a transportation service.
    #[serde(default, rename = "PlannedArrivalTransportEvent")]
    pub planned_arrival_transport_event: Option<crate::TransportEvent>,
/// A waypoint planned by the party providing a transportation service.
    #[serde(default, rename = "PlannedWaypointTransportEvent")]
    pub planned_waypoint_transport_event: Vec<crate::TransportEvent>,
/// The actual departure from a specific location during a transportation service.
    #[serde(default, rename = "ActualDepartureTransportEvent")]
    pub actual_departure_transport_event: Option<crate::TransportEvent>,
/// The location of an actual waypoint during a transportation service.
    #[serde(default, rename = "ActualWaypointTransportEvent")]
    pub actual_waypoint_transport_event: Option<crate::TransportEvent>,
/// The actual arrival at a specific location during a transportation service.
    #[serde(default, rename = "ActualArrivalTransportEvent")]
    pub actual_arrival_transport_event: Option<crate::TransportEvent>,
/// A additional significant occurrence in the course of this shipment of goods that is not defined
/// elsewhere in this Shipment Stage.
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: Vec<crate::TransportEvent>,
/// Describes an estimated departure at a location during a transport service.
    #[serde(default, rename = "EstimatedDepartureTransportEvent")]
    pub estimated_departure_transport_event: Option<crate::TransportEvent>,
/// Describes an estimated arrival at a location during a transport service.
    #[serde(default, rename = "EstimatedArrivalTransportEvent")]
    pub estimated_arrival_transport_event: Option<crate::TransportEvent>,
/// A person who travels in a conveyance without participating in its operation.
    #[serde(default, rename = "PassengerPerson")]
    pub passenger_person: Vec<crate::Person>,
/// Describes a person responsible for driving the transport means.
    #[serde(default, rename = "DriverPerson")]
    pub driver_person: Vec<crate::Person>,
/// Describes a person being responsible for providing the required administrative reporting relating to
/// a transport.
    #[serde(default, rename = "ReportingPerson")]
    pub reporting_person: Option<crate::Person>,
/// A person operating or serving aboard a transport means.
    #[serde(default, rename = "CrewMemberPerson")]
    pub crew_member_person: Vec<crate::Person>,
/// The person on board the vessel, accountable to the master, designated by the company as responsible
/// for the security of the ship, including implementation and maintenance of the ship security plan and
/// for the liaison with the company security officer and the port facility security officers.
    #[serde(default, rename = "SecurityOfficerPerson")]
    pub security_officer_person: Option<crate::Person>,
/// The person responsible for the ship's safe and efficient operation, including cargo operations,
/// navigation, crew management and for ensuring that the vessel complies with local and international
/// laws, as well as company and flag state policies.
    #[serde(default, rename = "MasterPerson")]
    pub master_person: Option<crate::Person>,
/// The person responsible for the health of the people aboard a ship at sea.
    #[serde(default, rename = "ShipsSurgeonPerson")]
    pub ships_surgeon_person: Option<crate::Person>,
/// A destination port call for this shipment stage.
    #[serde(default, rename = "DestinationPortCall")]
    pub destination_port_call: Option<crate::PortCall>,
/// The ship store articles for this shipment stage.
    #[serde(default, rename = "ShipStoreArticle")]
    pub ship_store_article: Vec<crate::ShipStoreArticle>,
/// The crew person effects for this shipment stage.
    #[serde(default, rename = "CrewPersonEffect")]
    pub crew_person_effect: Vec<crate::CrewPersonEffect>,
/// The maritime waste for this shipment stage.
    #[serde(default, rename = "MaritimeWaste")]
    pub maritime_waste: Vec<crate::MaritimeWaste>,
/// A ballast water summary for this shipment stage.
    #[serde(default, rename = "BallastWaterSummary")]
    pub ballast_water_summary: Option<crate::BallastWaterSummary>,
/// The ISPS (International Ship and Port Facility Security Code) requirements for this shipment stage.
    #[serde(default, rename = "ISPSRequirements")]
    pub isps_requirements: Option<crate::IspsRequirements>,
/// A maritime declaration of health for this shipment stage.
    #[serde(default, rename = "MaritimeHealthDeclaration")]
    pub maritime_health_declaration: Option<crate::MaritimeHealthDeclaration>,
/// One or more fuel consumptions of this shipment stage.
    #[serde(default, rename = "FuelConsumption")]
    pub fuel_consumption: Vec<crate::FuelConsumption>,
}
