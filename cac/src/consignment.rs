#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an identifiable collection of one or more goods items to be transported between
/// the consignor and the consignee. This information may be defined within a transport contract. A
/// consignment may comprise more than one shipment (e.g., when consolidated by a freight forwarder).
///
/// UBL Dictionary Entry Name: `Consignment. Details`
///
/// Generated from XSD type `ConsignmentType`.
pub struct Consignment {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier assigned to a collection of goods for both import and export.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// An identifier for this consignment, assigned by the carrier.
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: Option<cct::Identifier>,
/// An identifier for this consignment, assigned by the consignee.
    #[serde(default, rename = "ConsigneeAssignedID")]
    pub consignee_assigned_id: Option<cct::Identifier>,
/// An identifier for this consignment, assigned by the consignor.
    #[serde(default, rename = "ConsignorAssignedID")]
    pub consignor_assigned_id: Option<cct::Identifier>,
/// An identifier for this consignment, assigned by the freight forwarder.
    #[serde(default, rename = "FreightForwarderAssignedID")]
    pub freight_forwarder_assigned_id: Option<cct::Identifier>,
/// An identifier for this consignment, assigned by the broker.
    #[serde(default, rename = "BrokerAssignedID")]
    pub broker_assigned_id: Option<cct::Identifier>,
/// An identifier for this consignment, assigned by the contracted carrier.
    #[serde(default, rename = "ContractedCarrierAssignedID")]
    pub contracted_carrier_assigned_id: Option<cct::Identifier>,
/// An identifier for this consignment, assigned by the performing carrier.
    #[serde(default, rename = "PerformingCarrierAssignedID")]
    pub performing_carrier_assigned_id: Option<cct::Identifier>,
/// A textual summary description of the consignment.
    #[serde(default, rename = "SummaryDescription")]
    pub summary_description: Vec<cct::Text>,
/// The total of all invoice amounts declared in this consignment.
    #[serde(default, rename = "TotalInvoiceAmount")]
    pub total_invoice_amount: Option<cct::Amount>,
/// The total declared value for customs purposes of all the goods in this consignment, regardless of
/// whether they are subject to the same customs procedure, tariff/statistical categorization, country
/// information, or duty regime.
    #[serde(default, rename = "DeclaredCustomsValueAmount")]
    pub declared_customs_value_amount: Option<cct::Amount>,
/// Text describing the tariff applied to this consignment.
    #[serde(default, rename = "TariffDescription")]
    pub tariff_description: Vec<cct::Text>,
/// A code signifying the tariff applied to this consignment.
    #[serde(default, rename = "TariffCode")]
    pub tariff_code: Option<cct::Code>,
/// The amount of the premium payable to an insurance company for insuring the goods contained in this
/// consignment.
    #[serde(default, rename = "InsurancePremiumAmount")]
    pub insurance_premium_amount: Option<cct::Amount>,
/// The total declared weight of the goods in this consignment, including packaging but excluding the
/// carrier's equipment.
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: Option<cct::Measure>,
/// The total net weight of all the goods items referred to as one consignment.
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: Option<cct::Measure>,
/// The total net weight of the goods in this consignment, exclusive of packaging.
    #[serde(default, rename = "NetNetWeightMeasure")]
    pub net_net_weight_measure: Option<cct::Measure>,
/// The weight upon which a charge is to be based.
    #[serde(default, rename = "ChargeableWeightMeasure")]
    pub chargeable_weight_measure: Option<cct::Measure>,
/// The total volume of the goods referred to as one consignment.
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: Option<cct::Measure>,
/// The total net volume of all goods items referred to as one consignment.
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: Option<cct::Measure>,
/// The total length in a means of transport or a piece of transport equipment which, given the width
/// and height of the transport means, will accommodate all of the consignments in a single
/// consolidation.
    #[serde(default, rename = "LoadingLengthMeasure")]
    pub loading_length_measure: Option<cct::Measure>,
/// Remarks concerning the complete consignment, to be printed on the transport document.
    #[serde(default, rename = "Remarks")]
    pub remarks: Vec<cct::Text>,
/// An indication that the transported goods in this consignment are subject to an international
/// regulation concerning the carriage of dangerous goods (true) or not (false).
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<udt::Indicator>,
/// An indication that the transported goods in this consignment are animal foodstuffs (true) or not
/// (false).
    #[serde(default, rename = "AnimalFoodIndicator")]
    pub animal_food_indicator: Option<udt::Indicator>,
/// An indication that the transported goods in this consignment are for human consumption (true) or not
/// (false).
    #[serde(default, rename = "HumanFoodIndicator")]
    pub human_food_indicator: Option<udt::Indicator>,
/// An indication that the transported goods are livestock (true) or not (false).
    #[serde(default, rename = "LivestockIndicator")]
    pub livestock_indicator: Option<udt::Indicator>,
/// An indication that the transported goods in this consignment are bulk cargoes (true) or not (false).
    #[serde(default, rename = "BulkCargoIndicator")]
    pub bulk_cargo_indicator: Option<udt::Indicator>,
/// An indication that the transported goods in this consignment are containerized cargoes (true) or not
/// (false).
    #[serde(default, rename = "ContainerizedIndicator")]
    pub containerized_indicator: Option<udt::Indicator>,
/// An indication that the transported goods in this consignment are general cargoes (true) or not
/// (false).
    #[serde(default, rename = "GeneralCargoIndicator")]
    pub general_cargo_indicator: Option<udt::Indicator>,
/// An indication that the transported goods in this consignment require special security (true) or not
/// (false).
    #[serde(default, rename = "SpecialSecurityIndicator")]
    pub special_security_indicator: Option<udt::Indicator>,
/// An indication that this consignment will be paid for by a third party (true) or not (false).
    #[serde(default, rename = "ThirdPartyPayerIndicator")]
    pub third_party_payer_indicator: Option<udt::Indicator>,
/// Service instructions to the carrier, expressed as text.
    #[serde(default, rename = "CarrierServiceInstructions")]
    pub carrier_service_instructions: Vec<cct::Text>,
/// Service instructions for customs clearance, expressed as text.
    #[serde(default, rename = "CustomsClearanceServiceInstructions")]
    pub customs_clearance_service_instructions: Vec<cct::Text>,
/// Service instructions for the forwarder, expressed as text.
    #[serde(default, rename = "ForwarderServiceInstructions")]
    pub forwarder_service_instructions: Vec<cct::Text>,
/// Special service instructions, expressed as text.
    #[serde(default, rename = "SpecialServiceInstructions")]
    pub special_service_instructions: Vec<cct::Text>,
/// A sequence identifier for this consignment.
    #[serde(default, rename = "SequenceID")]
    pub sequence_id: Option<cct::Identifier>,
/// A code signifying the priority or level of service required for this consignment.
    #[serde(default, rename = "ShippingPriorityLevelCode")]
    pub shipping_priority_level_code: Option<cct::Code>,
/// The handling required for this consignment, expressed as a code.
    #[serde(default, rename = "HandlingCode")]
    pub handling_code: Vec<cct::Code>,
/// The handling required for this consignment, expressed as text.
    #[serde(default, rename = "HandlingInstructions")]
    pub handling_instructions: Vec<cct::Text>,
/// Free-form text pertinent to this consignment, conveying information that is not contained explicitly
/// in other structures.
    #[serde(default, rename = "Information")]
    pub information: Vec<cct::Text>,
/// The total number of goods items in this consignment.
    #[serde(default, rename = "TotalGoodsItemQuantity")]
    pub total_goods_item_quantity: Option<cct::Quantity>,
/// The number of pieces of transport handling equipment (pallets, boxes, cases, etc.) in this
/// consignment.
    #[serde(default, rename = "TotalTransportHandlingUnitQuantity")]
    pub total_transport_handling_unit_quantity: Option<cct::Quantity>,
/// The amount covered by insurance for this consignment.
    #[serde(default, rename = "InsuranceValueAmount")]
    pub insurance_value_amount: Option<cct::Amount>,
/// The value of this consignment, declared by the shipper or his agent solely for the purpose of
/// varying the carrier's level of liability from that provided in the contract of carriage, in case of
/// loss or damage to goods or delayed delivery.
    #[serde(default, rename = "DeclaredForCarriageValueAmount")]
    pub declared_for_carriage_value_amount: Option<cct::Amount>,
/// The value, declared for statistical purposes, of those goods in this consignment that have the same
/// statistical heading.
    #[serde(default, rename = "DeclaredStatisticsValueAmount")]
    pub declared_statistics_value_amount: Option<cct::Amount>,
/// The monetary amount that has to be or has been paid as calculated under the applicable trade
/// delivery.
    #[serde(default, rename = "FreeOnBoardValueAmount")]
    pub free_on_board_value_amount: Option<cct::Amount>,
/// Special instructions relating to this consignment.
    #[serde(default, rename = "SpecialInstructions")]
    pub special_instructions: Vec<cct::Text>,
/// An indicator that this consignment has been split in transit (true) or not (false).
    #[serde(default, rename = "SplitConsignmentIndicator")]
    pub split_consignment_indicator: Option<udt::Indicator>,
/// A set of delivery instructions relating to this consignment.
    #[serde(default, rename = "DeliveryInstructions")]
    pub delivery_instructions: Vec<cct::Text>,
/// The count in this consignment considering goods items, child consignments, shipments
    #[serde(default, rename = "ConsignmentQuantity")]
    pub consignment_quantity: Option<cct::Quantity>,
/// An indicator that this consignment can be consolidated (true) or not (false).
    #[serde(default, rename = "ConsolidatableIndicator")]
    pub consolidatable_indicator: Option<udt::Indicator>,
/// Instructions regarding haulage of this consignment, expressed as text.
    #[serde(default, rename = "HaulageInstructions")]
    pub haulage_instructions: Vec<cct::Text>,
/// An identifier for the loading sequence of this consignment.
    #[serde(default, rename = "LoadingSequenceID")]
    pub loading_sequence_id: Option<cct::Identifier>,
/// The quantity of (consolidated) child consignments
    #[serde(default, rename = "ChildConsignmentQuantity")]
    pub child_consignment_quantity: Option<cct::Quantity>,
/// The total number of packages associated with a Consignment.
    #[serde(default, rename = "TotalPackagesQuantity")]
    pub total_packages_quantity: Option<cct::Quantity>,
/// A consolidated shipment (a shipment created by an act of consolidation).
    #[serde(default, rename = "ConsolidatedShipment")]
    pub consolidated_shipment: Vec<Shipment>,
/// A class describing identifiers or references relating to customs procedures.
    #[serde(default, rename = "CustomsDeclaration")]
    pub customs_declaration: Vec<CustomsDeclaration>,
/// The pickup of this consignment requested by the party requesting a transportation service (the
/// transport user).
    #[serde(default, rename = "RequestedPickupTransportEvent")]
    pub requested_pickup_transport_event: Option<TransportEvent>,
/// The delivery of this consignment requested by the party requesting a transportation service (the
/// transport user).
    #[serde(default, rename = "RequestedDeliveryTransportEvent")]
    pub requested_delivery_transport_event: Option<TransportEvent>,
/// The pickup of this consignment planned by the party responsible for providing the transportation
/// service (the transport service provider).
    #[serde(default, rename = "PlannedPickupTransportEvent")]
    pub planned_pickup_transport_event: Option<TransportEvent>,
/// The delivery of this consignment planned by the party responsible for providing the transportation
/// service (the transport service provider).
    #[serde(default, rename = "PlannedDeliveryTransportEvent")]
    pub planned_delivery_transport_event: Option<TransportEvent>,
/// The actual pickup of this consignment by the party responsible for providing the transportation
/// service (the transport service provider).
    #[serde(default, rename = "ActualPickupTransportEvent")]
    pub actual_pickup_transport_event: Option<TransportEvent>,
/// The actual delivery of this consignment by the party responsible for providing the transportation
/// service (the transport service provider).
    #[serde(default, rename = "ActualDeliveryTransportEvent")]
    pub actual_delivery_transport_event: Option<TransportEvent>,
/// The status of a particular condition associated with this consignment.
    #[serde(default, rename = "Status")]
    pub status: Vec<Status>,
/// One of the child consignments of which a consolidated consignment is composed.
    #[serde(default, rename = "ChildConsignment")]
    pub child_consignment: Vec<Consignment>,
/// The Party who receives the goods.
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: Option<Party>,
/// The Party who exports the goods or has similar right of disposal over them at the time of export.
    #[serde(default, rename = "ExporterParty")]
    pub exporter_party: Option<Party>,
/// The Party who is reponsible for sending the goods.
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: Option<Party>,
/// The Party who imports the goods, or on whose behalf the goods are being imported.
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: Option<Party>,
/// The Party who provides the transport of goods between named points.
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: Option<Party>,
/// The Party who combines individual smaller consignments into a single larger shipment (a so-called
/// consolidated consignment or shipment) which is sent to a counterpart who mirrors the consolidator's
/// activity by dividing the consolidated consignment into its original components.
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<Party>,
/// The Party who is notified upon arrival of Goods and when special occurrences (usually pre-defined)
/// take place during a transportation service.
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: Option<Party>,
/// The Party who originally sends this Consignment.
    #[serde(default, rename = "OriginalDespatchParty")]
    pub original_despatch_party: Option<Party>,
/// The final delivery party for this consignment.
    #[serde(default, rename = "FinalDeliveryParty")]
    pub final_delivery_party: Option<Party>,
/// The Party who performs the carriage of this Consignment.
    #[serde(default, rename = "PerformingCarrierParty")]
    pub performing_carrier_party: Option<Party>,
/// The Party who subtitutes the carrier of this Consignment.
    #[serde(default, rename = "SubstituteCarrierParty")]
    pub substitute_carrier_party: Option<Party>,
/// The Party who operates the logistics for this Consignment.
    #[serde(default, rename = "LogisticsOperatorParty")]
    pub logistics_operator_party: Option<Party>,
/// The Party who provides transport advice in this Consignment.
    #[serde(default, rename = "TransportAdvisorParty")]
    pub transport_advisor_party: Option<Party>,
/// The Party who is notified of a Hazardous Item in this Consignment.
    #[serde(default, rename = "HazardousItemNotificationParty")]
    pub hazardous_item_notification_party: Option<Party>,
/// The Party who holds the insurance for this Consignment.
    #[serde(default, rename = "InsuranceParty")]
    pub insurance_party: Option<Party>,
/// The Party who holds the mortgage for this Consignment.
    #[serde(default, rename = "MortgageHolderParty")]
    pub mortgage_holder_party: Option<Party>,
/// The Party who holds the Bill of Lading for this Consignment.
    #[serde(default, rename = "BillOfLadingHolderParty")]
    pub bill_of_lading_holder_party: Option<Party>,
/// The country from which the goods in this consignment were originally exported, without any
/// commercial transaction taking place in intermediate countries.
    #[serde(default, rename = "OriginalDepartureCountry")]
    pub original_departure_country: Option<Country>,
/// The country in which the goods in this consignment are to be delivered to the final consignee or
/// buyer.
    #[serde(default, rename = "FinalDestinationCountry")]
    pub final_destination_country: Option<Country>,
/// One of the countries through which goods or passengers in this consignment are routed between the
/// country of original departure and the country of final destination.
    #[serde(default, rename = "TransitCountry")]
    pub transit_country: Vec<Country>,
/// A transport contract relating to this consignment.
    #[serde(default, rename = "TransportContract")]
    pub transport_contract: Option<Contract>,
/// A class for describing any additional significant occurrences or happenings related to the
/// transportation of goods not specified elsewhere in this Consignment.
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: Vec<TransportEvent>,
/// The service for pickup from the consignor under the transport contract for this consignment.
    #[serde(default, rename = "OriginalDespatchTransportationService")]
    pub original_despatch_transportation_service: Option<TransportationService>,
/// The service for delivery to the consignee under the transport contract for this consignment.
    #[serde(default, rename = "FinalDeliveryTransportationService")]
    pub final_delivery_transportation_service: Option<TransportationService>,
/// The conditions agreed upon between a seller and a buyer with regard to the delivery of goods and/or
/// services (e.g., CIF, FOB, or EXW from the INCOTERMS Terms of Delivery).
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Option<DeliveryTerms>,
/// The terms of payment between the parties (such as logistics service client, logistics service
/// provider) in a transaction.
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Option<PaymentTerms>,
/// The terms of payment that apply to the collection of this consignment.
    #[serde(default, rename = "CollectPaymentTerms")]
    pub collect_payment_terms: Option<PaymentTerms>,
/// The terms of payment for disbursement.
    #[serde(default, rename = "DisbursementPaymentTerms")]
    pub disbursement_payment_terms: Option<PaymentTerms>,
/// The terms of payment for prepayment.
    #[serde(default, rename = "PrepaidPaymentTerms")]
    pub prepaid_payment_terms: Option<PaymentTerms>,
/// A cost incurred by the shipper in moving goods, by whatever means, from one place to another under
/// the terms of the contract of carriage for this consignment. In addition to transport costs, this may
/// include such elements as packing, documentation, loading, unloading, and insurance to the extent
/// that they relate to the freight costs.
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: Vec<AllowanceCharge>,
/// A charge for extra allowance.
    #[serde(default, rename = "ExtraAllowanceCharge")]
    pub extra_allowance_charge: Vec<AllowanceCharge>,
/// A shipment stage during main carriage.
    #[serde(default, rename = "MainCarriageShipmentStage")]
    pub main_carriage_shipment_stage: Vec<ShipmentStage>,
/// A shipment stage during precarriage (usually refers to movement activity that takes place prior to
/// the container being loaded at a port of loading).
    #[serde(default, rename = "PreCarriageShipmentStage")]
    pub pre_carriage_shipment_stage: Vec<ShipmentStage>,
/// A shipment stage during on-carriage (usually refers to movement activity that takes place after the
/// container is discharged at a port of discharge).
    #[serde(default, rename = "OnCarriageShipmentStage")]
    pub on_carriage_shipment_stage: Vec<ShipmentStage>,
/// A transport handling unit used for loose and containerized goods.
    #[serde(default, rename = "TransportHandlingUnit")]
    pub transport_handling_unit: Vec<TransportHandlingUnit>,
/// The first arrival location in a transport. This would be a port for sea, an airport for air, a
/// terminal for rail, or a border post for land crossing.
    #[serde(default, rename = "FirstArrivalPortLocation")]
    pub first_arrival_port_location: Option<Location>,
/// The final exporting location in a transport. This would be a port for sea, an airport for air, a
/// terminal for rail, or a border post for land crossing.
    #[serde(default, rename = "LastExitPortLocation")]
    pub last_exit_port_location: Option<Location>,
/// The customs office or offices indicated in the authorisation as empowered to accept declarations
/// entering goods for the arrangements.
    #[serde(default, rename = "OfficeOfEntryLocation")]
    pub office_of_entry_location: Option<Location>,
/// A location that is involved in the subsequent entry of goods in a consignment. This could mean a
/// customs office or facility where goods are processed after their initial entry point, possibly for
/// further clearance, inspection, or transit.
    #[serde(default, rename = "OfficeOfSubSequentiallyEntryLocation")]
    pub office_of_sub_sequentially_entry_location: Option<Location>,
/// The customs office of the actual exit of the goods at which the goods are placed in the export
/// procedure and released for exit.
    #[serde(default, rename = "OfficeOfExitLocation")]
    pub office_of_exit_location: Option<Location>,
/// A Customs Office where the customs declaration placing goods under transit is accepted.
    #[serde(default, rename = "OfficeOfDepartureLocation")]
    pub office_of_departure_location: Option<Location>,
/// Any customs office at which a customs transit operation is terminated.
    #[serde(default, rename = "OfficeOfDestinationLocation")]
    pub office_of_destination_location: Option<Location>,
/// The customs office where the formalities for assigning goods brought into the customs territory of
/// the Community to a customs-approved treatment or use are to be carried out.
    #[serde(default, rename = "OfficeOfImportLocation")]
    pub office_of_import_location: Option<Location>,
/// The customs office at which an export declaration or a re-export declaration is made.
    #[serde(default, rename = "OfficeOfExportLocation")]
    pub office_of_export_location: Option<Location>,
/// A location that finds the reference numbers for the departure, transit and destination offices.
    #[serde(default, rename = "OfficeOfTransitLocation")]
    pub office_of_transit_location: Vec<Location>,
/// A reference to a document related to or relevant for this consignment.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
/// One or more environmental emissions of this consignment.
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<EnvironmentalEmission>,
/// One or more Insurance Policies that apply to this consignment.
    #[serde(default, rename = "InsurancePolicy")]
    pub insurance_policy: Vec<InsurancePolicy>,
}
