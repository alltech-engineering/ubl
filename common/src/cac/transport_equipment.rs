#[derive(Debug, Deserialize, Serialize)]
pub struct TransportEquipment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ReferencedConsignmentID")]
    pub referenced_consignment_id: Vec<super::cct::IdentifierType>,
    #[serde(default, rename = "TransportEquipmentTypeCode")]
    pub transport_equipment_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ProviderTypeCode")]
    pub provider_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "OwnerTypeCode")]
    pub owner_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "SizeTypeCode")]
    pub size_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "DispositionCode")]
    pub disposition_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "FullnessIndicationCode")]
    pub fullness_indication_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "RefrigerationOnIndicator")]
    pub refrigeration_on_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "Information")]
    pub information: Vec<super::cct::TextType>,
    #[serde(default, rename = "ReturnabilityIndicator")]
    pub returnability_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "LegalStatusIndicator")]
    pub legal_status_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "AirFlowPercent")]
    pub air_flow_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "HumidityPercent")]
    pub humidity_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "AnimalFoodApprovedIndicator")]
    pub animal_food_approved_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "HumanFoodApprovedIndicator")]
    pub human_food_approved_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DangerousGoodsApprovedIndicator")]
    pub dangerous_goods_approved_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "RefrigeratedIndicator")]
    pub refrigerated_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "Characteristics")]
    pub characteristics: Option<super::cct::TextType>,
    #[serde(default, rename = "DamageRemarks")]
    pub damage_remarks: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "SpecialTransportRequirements")]
    pub special_transport_requirements: Vec<super::cct::TextType>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "TareWeightMeasure")]
    pub tare_weight_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "TrackingDeviceCode")]
    pub tracking_device_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "PowerIndicator")]
    pub power_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TraceID")]
    pub trace_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "StowagePositionID")]
    pub stowage_position_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<Dimension>,
    #[serde(default, rename = "TransportEquipmentSeal")]
    pub transport_equipment_seal: Vec<TransportEquipmentSeal>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<Temperature>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<Temperature>,
    #[serde(default, rename = "ProviderParty")]
    pub provider_party: Option<Party>,
    #[serde(default, rename = "LoadingProofParty")]
    pub loading_proof_party: Option<Party>,
    #[serde(default, rename = "SupplierParty")]
    pub supplier_party: Option<SupplierParty>,
    #[serde(default, rename = "OwnerParty")]
    pub owner_party: Option<Party>,
    #[serde(default, rename = "OperatingParty")]
    pub operating_party: Option<Party>,
    #[serde(default, rename = "LoadingLocation")]
    pub loading_location: Option<Location>,
    #[serde(default, rename = "UnloadingLocation")]
    pub unloading_location: Option<Location>,
    #[serde(default, rename = "StorageLocation")]
    pub storage_location: Option<Location>,
    #[serde(default, rename = "PositioningTransportEvent")]
    pub positioning_transport_event: Vec<TransportEvent>,
    #[serde(default, rename = "QuarantineTransportEvent")]
    pub quarantine_transport_event: Vec<TransportEvent>,
    #[serde(default, rename = "DeliveryTransportEvent")]
    pub delivery_transport_event: Vec<TransportEvent>,
    #[serde(default, rename = "PickupTransportEvent")]
    pub pickup_transport_event: Vec<TransportEvent>,
    #[serde(default, rename = "HandlingTransportEvent")]
    pub handling_transport_event: Vec<TransportEvent>,
    #[serde(default, rename = "LoadingTransportEvent")]
    pub loading_transport_event: Vec<TransportEvent>,
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: Vec<TransportEvent>,
    #[serde(default, rename = "ApplicableTransportMeans")]
    pub applicable_transport_means: Option<TransportMeans>,
    #[serde(default, rename = "HaulageTradingTerms")]
    pub haulage_trading_terms: Vec<TradingTerms>,
    #[serde(default, rename = "HazardousGoodsTransit")]
    pub hazardous_goods_transit: Vec<HazardousGoodsTransit>,
    #[serde(default, rename = "PackagedTransportHandlingUnit")]
    pub packaged_transport_handling_unit: Vec<TransportHandlingUnit>,
    #[serde(default, rename = "ServiceAllowanceCharge")]
    pub service_allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "AttachedTransportEquipment")]
    pub attached_transport_equipment: Vec<TransportEquipment>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Option<Box<Delivery>>,
    #[serde(default, rename = "Pickup")]
    pub pickup: Option<Pickup>,
    #[serde(default, rename = "Despatch")]
    pub despatch: Option<Despatch>,
    #[serde(default, rename = "ShipmentDocumentReference")]
    pub shipment_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "ContainedInTransportEquipment")]
    pub contained_in_transport_equipment: Vec<TransportEquipment>,
    #[serde(default, rename = "Package")]
    pub package: Vec<Package>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Vec<GoodsItem>,
    #[serde(default, rename = "VerifiedGrossMass")]
    pub verified_gross_mass: Option<VerifiedGrossMass>,
    #[serde(default, rename = "LoadedHazardousItem")]
    pub loaded_hazardous_item: Vec<HazardousItem>,
}
