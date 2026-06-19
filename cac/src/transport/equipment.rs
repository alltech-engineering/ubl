#[derive(Debug, Deserialize, Serialize)]
pub struct TransportEquipment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "ReferencedConsignmentID")]
    pub referenced_consignment_id: Vec<cct::Identifier>,
    #[serde(default, rename = "TransportEquipmentTypeCode")]
    pub transport_equipment_type_code: Option<cct::Code>,
    #[serde(default, rename = "ProviderTypeCode")]
    pub provider_type_code: Option<cct::Code>,
    #[serde(default, rename = "OwnerTypeCode")]
    pub owner_type_code: Option<cct::Code>,
    #[serde(default, rename = "SizeTypeCode")]
    pub size_type_code: Option<cct::Code>,
    #[serde(default, rename = "DispositionCode")]
    pub disposition_code: Option<cct::Code>,
    #[serde(default, rename = "FullnessIndicationCode")]
    pub fullness_indication_code: Option<cct::Code>,
    #[serde(default, rename = "RefrigerationOnIndicator")]
    pub refrigeration_on_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "Information")]
    pub information: Vec<cct::Text>,
    #[serde(default, rename = "ReturnabilityIndicator")]
    pub returnability_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "LegalStatusIndicator")]
    pub legal_status_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "AirFlowPercent")]
    pub air_flow_percent: Option<cct::Numeric>,
    #[serde(default, rename = "HumidityPercent")]
    pub humidity_percent: Option<cct::Numeric>,
    #[serde(default, rename = "AnimalFoodApprovedIndicator")]
    pub animal_food_approved_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "HumanFoodApprovedIndicator")]
    pub human_food_approved_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "DangerousGoodsApprovedIndicator")]
    pub dangerous_goods_approved_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "RefrigeratedIndicator")]
    pub refrigerated_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "Characteristics")]
    pub characteristics: Option<cct::Text>,
    #[serde(default, rename = "DamageRemarks")]
    pub damage_remarks: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "SpecialTransportRequirements")]
    pub special_transport_requirements: Vec<cct::Text>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: Option<cct::Measure>,
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: Option<cct::Measure>,
    #[serde(default, rename = "TareWeightMeasure")]
    pub tare_weight_measure: Option<cct::Measure>,
    #[serde(default, rename = "TrackingDeviceCode")]
    pub tracking_device_code: Option<cct::Code>,
    #[serde(default, rename = "PowerIndicator")]
    pub power_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "TraceID")]
    pub trace_id: Option<cct::Identifier>,
    #[serde(default, rename = "StowagePositionID")]
    pub stowage_position_id: Option<cct::Identifier>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<crate::Dimension>,
    #[serde(default, rename = "TransportEquipmentSeal")]
    pub transport_equipment_seal: Vec<TransportEquipmentSeal>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<crate::Temperature>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<crate::Temperature>,
    #[serde(default, rename = "ProviderParty")]
    pub provider_party: Option<crate::Party>,
    #[serde(default, rename = "LoadingProofParty")]
    pub loading_proof_party: Option<crate::Party>,
    #[serde(default, rename = "SupplierParty")]
    pub supplier_party: Option<crate::SupplierParty>,
    #[serde(default, rename = "OwnerParty")]
    pub owner_party: Option<crate::Party>,
    #[serde(default, rename = "OperatingParty")]
    pub operating_party: Option<crate::Party>,
    #[serde(default, rename = "LoadingLocation")]
    pub loading_location: Option<crate::Location>,
    #[serde(default, rename = "UnloadingLocation")]
    pub unloading_location: Option<crate::Location>,
    #[serde(default, rename = "StorageLocation")]
    pub storage_location: Option<crate::Location>,
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
    pub haulage_trading_terms: Vec<crate::TradingTerms>,
    #[serde(default, rename = "HazardousGoodsTransit")]
    pub hazardous_goods_transit: Vec<crate::HazardousGoodsTransit>,
    #[serde(default, rename = "PackagedTransportHandlingUnit")]
    pub packaged_transport_handling_unit: Vec<TransportHandlingUnit>,
    #[serde(default, rename = "ServiceAllowanceCharge")]
    pub service_allowance_charge: Vec<crate::AllowanceCharge>,
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: Vec<crate::AllowanceCharge>,
    #[serde(default, rename = "AttachedTransportEquipment")]
    pub attached_transport_equipment: Vec<TransportEquipment>,
    #[serde(default, rename = "Delivery")]
    pub delivery: Option<Box<crate::Delivery>>,
    #[serde(default, rename = "Pickup")]
    pub pickup: Option<crate::Pickup>,
    #[serde(default, rename = "Despatch")]
    pub despatch: Option<crate::Despatch>,
    #[serde(default, rename = "ShipmentDocumentReference")]
    pub shipment_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "ContainedInTransportEquipment")]
    pub contained_in_transport_equipment: Vec<TransportEquipment>,
    #[serde(default, rename = "Package")]
    pub package: Vec<crate::Package>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Vec<crate::GoodsItem>,
    #[serde(default, rename = "VerifiedGrossMass")]
    pub verified_gross_mass: Option<crate::VerifiedGrossMass>,
    #[serde(default, rename = "LoadedHazardousItem")]
    pub loaded_hazardous_item: Vec<crate::HazardousItem>,
}
