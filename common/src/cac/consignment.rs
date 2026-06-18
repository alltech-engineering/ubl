#[derive(Debug, Deserialize, Serialize)]
pub struct Consignment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ConsigneeAssignedID")]
    pub consignee_assigned_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ConsignorAssignedID")]
    pub consignor_assigned_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "FreightForwarderAssignedID")]
    pub freight_forwarder_assigned_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "BrokerAssignedID")]
    pub broker_assigned_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ContractedCarrierAssignedID")]
    pub contracted_carrier_assigned_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PerformingCarrierAssignedID")]
    pub performing_carrier_assigned_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SummaryDescription")]
    pub summary_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "TotalInvoiceAmount")]
    pub total_invoice_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredCustomsValueAmount")]
    pub declared_customs_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TariffDescription")]
    pub tariff_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "TariffCode")]
    pub tariff_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "InsurancePremiumAmount")]
    pub insurance_premium_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetNetWeightMeasure")]
    pub net_net_weight_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "ChargeableWeightMeasure")]
    pub chargeable_weight_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "LoadingLengthMeasure")]
    pub loading_length_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "Remarks")]
    pub remarks: Vec<super::cct::TextType>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "AnimalFoodIndicator")]
    pub animal_food_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "HumanFoodIndicator")]
    pub human_food_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "LivestockIndicator")]
    pub livestock_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "BulkCargoIndicator")]
    pub bulk_cargo_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ContainerizedIndicator")]
    pub containerized_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "GeneralCargoIndicator")]
    pub general_cargo_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SpecialSecurityIndicator")]
    pub special_security_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ThirdPartyPayerIndicator")]
    pub third_party_payer_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "CarrierServiceInstructions")]
    pub carrier_service_instructions: Vec<super::cct::TextType>,
    #[serde(default, rename = "CustomsClearanceServiceInstructions")]
    pub customs_clearance_service_instructions: Vec<super::cct::TextType>,
    #[serde(default, rename = "ForwarderServiceInstructions")]
    pub forwarder_service_instructions: Vec<super::cct::TextType>,
    #[serde(default, rename = "SpecialServiceInstructions")]
    pub special_service_instructions: Vec<super::cct::TextType>,
    #[serde(default, rename = "SequenceID")]
    pub sequence_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ShippingPriorityLevelCode")]
    pub shipping_priority_level_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "HandlingCode")]
    pub handling_code: Vec<super::cct::CodeType>,
    #[serde(default, rename = "HandlingInstructions")]
    pub handling_instructions: Vec<super::cct::TextType>,
    #[serde(default, rename = "Information")]
    pub information: Vec<super::cct::TextType>,
    #[serde(default, rename = "TotalGoodsItemQuantity")]
    pub total_goods_item_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "TotalTransportHandlingUnitQuantity")]
    pub total_transport_handling_unit_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "InsuranceValueAmount")]
    pub insurance_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredForCarriageValueAmount")]
    pub declared_for_carriage_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredStatisticsValueAmount")]
    pub declared_statistics_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "FreeOnBoardValueAmount")]
    pub free_on_board_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "SpecialInstructions")]
    pub special_instructions: Vec<super::cct::TextType>,
    #[serde(default, rename = "SplitConsignmentIndicator")]
    pub split_consignment_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DeliveryInstructions")]
    pub delivery_instructions: Vec<super::cct::TextType>,
    #[serde(default, rename = "ConsignmentQuantity")]
    pub consignment_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ConsolidatableIndicator")]
    pub consolidatable_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "HaulageInstructions")]
    pub haulage_instructions: Vec<super::cct::TextType>,
    #[serde(default, rename = "LoadingSequenceID")]
    pub loading_sequence_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ChildConsignmentQuantity")]
    pub child_consignment_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "TotalPackagesQuantity")]
    pub total_packages_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ConsolidatedShipment")]
    pub consolidated_shipment: Vec<Shipment>,
    #[serde(default, rename = "CustomsDeclaration")]
    pub customs_declaration: Vec<CustomsDeclaration>,
    #[serde(default, rename = "RequestedPickupTransportEvent")]
    pub requested_pickup_transport_event: Option<TransportEvent>,
    #[serde(default, rename = "RequestedDeliveryTransportEvent")]
    pub requested_delivery_transport_event: Option<TransportEvent>,
    #[serde(default, rename = "PlannedPickupTransportEvent")]
    pub planned_pickup_transport_event: Option<TransportEvent>,
    #[serde(default, rename = "PlannedDeliveryTransportEvent")]
    pub planned_delivery_transport_event: Option<TransportEvent>,
    #[serde(default, rename = "ActualPickupTransportEvent")]
    pub actual_pickup_transport_event: Option<TransportEvent>,
    #[serde(default, rename = "ActualDeliveryTransportEvent")]
    pub actual_delivery_transport_event: Option<TransportEvent>,
    #[serde(default, rename = "Status")]
    pub status: Vec<Status>,
    #[serde(default, rename = "ChildConsignment")]
    pub child_consignment: Vec<Consignment>,
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: Option<Party>,
    #[serde(default, rename = "ExporterParty")]
    pub exporter_party: Option<Party>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: Option<Party>,
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: Option<Party>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: Option<Party>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<Party>,
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: Option<Party>,
    #[serde(default, rename = "OriginalDespatchParty")]
    pub original_despatch_party: Option<Party>,
    #[serde(default, rename = "FinalDeliveryParty")]
    pub final_delivery_party: Option<Party>,
    #[serde(default, rename = "PerformingCarrierParty")]
    pub performing_carrier_party: Option<Party>,
    #[serde(default, rename = "SubstituteCarrierParty")]
    pub substitute_carrier_party: Option<Party>,
    #[serde(default, rename = "LogisticsOperatorParty")]
    pub logistics_operator_party: Option<Party>,
    #[serde(default, rename = "TransportAdvisorParty")]
    pub transport_advisor_party: Option<Party>,
    #[serde(default, rename = "HazardousItemNotificationParty")]
    pub hazardous_item_notification_party: Option<Party>,
    #[serde(default, rename = "InsuranceParty")]
    pub insurance_party: Option<Party>,
    #[serde(default, rename = "MortgageHolderParty")]
    pub mortgage_holder_party: Option<Party>,
    #[serde(default, rename = "BillOfLadingHolderParty")]
    pub bill_of_lading_holder_party: Option<Party>,
    #[serde(default, rename = "OriginalDepartureCountry")]
    pub original_departure_country: Option<Country>,
    #[serde(default, rename = "FinalDestinationCountry")]
    pub final_destination_country: Option<Country>,
    #[serde(default, rename = "TransitCountry")]
    pub transit_country: Vec<Country>,
    #[serde(default, rename = "TransportContract")]
    pub transport_contract: Option<Contract>,
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: Vec<TransportEvent>,
    #[serde(default, rename = "OriginalDespatchTransportationService")]
    pub original_despatch_transportation_service: Option<TransportationService>,
    #[serde(default, rename = "FinalDeliveryTransportationService")]
    pub final_delivery_transportation_service: Option<TransportationService>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Option<DeliveryTerms>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Option<PaymentTerms>,
    #[serde(default, rename = "CollectPaymentTerms")]
    pub collect_payment_terms: Option<PaymentTerms>,
    #[serde(default, rename = "DisbursementPaymentTerms")]
    pub disbursement_payment_terms: Option<PaymentTerms>,
    #[serde(default, rename = "PrepaidPaymentTerms")]
    pub prepaid_payment_terms: Option<PaymentTerms>,
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "ExtraAllowanceCharge")]
    pub extra_allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "MainCarriageShipmentStage")]
    pub main_carriage_shipment_stage: Vec<ShipmentStage>,
    #[serde(default, rename = "PreCarriageShipmentStage")]
    pub pre_carriage_shipment_stage: Vec<ShipmentStage>,
    #[serde(default, rename = "OnCarriageShipmentStage")]
    pub on_carriage_shipment_stage: Vec<ShipmentStage>,
    #[serde(default, rename = "TransportHandlingUnit")]
    pub transport_handling_unit: Vec<TransportHandlingUnit>,
    #[serde(default, rename = "FirstArrivalPortLocation")]
    pub first_arrival_port_location: Option<Location>,
    #[serde(default, rename = "LastExitPortLocation")]
    pub last_exit_port_location: Option<Location>,
    #[serde(default, rename = "OfficeOfEntryLocation")]
    pub office_of_entry_location: Option<Location>,
    #[serde(default, rename = "OfficeOfSubSequentiallyEntryLocation")]
    pub office_of_sub_sequentially_entry_location: Option<Location>,
    #[serde(default, rename = "OfficeOfExitLocation")]
    pub office_of_exit_location: Option<Location>,
    #[serde(default, rename = "OfficeOfDepartureLocation")]
    pub office_of_departure_location: Option<Location>,
    #[serde(default, rename = "OfficeOfDestinationLocation")]
    pub office_of_destination_location: Option<Location>,
    #[serde(default, rename = "OfficeOfImportLocation")]
    pub office_of_import_location: Option<Location>,
    #[serde(default, rename = "OfficeOfExportLocation")]
    pub office_of_export_location: Option<Location>,
    #[serde(default, rename = "OfficeOfTransitLocation")]
    pub office_of_transit_location: Vec<Location>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<EnvironmentalEmission>,
    #[serde(default, rename = "InsurancePolicy")]
    pub insurance_policy: Vec<InsurancePolicy>,
}
