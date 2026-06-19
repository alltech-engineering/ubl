#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a piece of equipment used to transport goods.
///
/// UBL Dictionary Entry Name: `Transport Equipment. Details`
///
/// Generated from XSD type `TransportEquipmentType`.
pub struct TransportEquipment {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this piece of transport equipment.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// An identifier for the consignment contained by this piece of transport equipment.
    #[serde(default, rename = "ReferencedConsignmentID")]
    pub referenced_consignment_id: Vec<cct::Identifier>,
/// A code signifying the type of this piece of transport equipment.
    #[serde(default, rename = "TransportEquipmentTypeCode")]
    pub transport_equipment_type_code: Option<cct::Code>,
/// A code signifying the type of provider of this piece of transport equipment.
    #[serde(default, rename = "ProviderTypeCode")]
    pub provider_type_code: Option<cct::Code>,
/// A code signifying the type of owner of this piece of transport equipment.
    #[serde(default, rename = "OwnerTypeCode")]
    pub owner_type_code: Option<cct::Code>,
/// A code signifying the size and type of this piece of piece of transport equipment. When the piece of
/// transport equipment is a shipping container, it is recommended to use ContainerSizeTypeCode for
/// validation.
    #[serde(default, rename = "SizeTypeCode")]
    pub size_type_code: Option<cct::Code>,
/// A code signifying the current disposition of this piece of transport equipment.
    #[serde(default, rename = "DispositionCode")]
    pub disposition_code: Option<cct::Code>,
/// A code signifying whether this piece of transport equipment is full, partially full, or empty.
    #[serde(default, rename = "FullnessIndicationCode")]
    pub fullness_indication_code: Option<cct::Code>,
/// An indicator that this piece of transport equipment's refrigeration is on (true) or off (false).
    #[serde(default, rename = "RefrigerationOnIndicator")]
    pub refrigeration_on_indicator: Option<udt::Indicator>,
/// Additional information about this piece of transport equipment.
    #[serde(default, rename = "Information")]
    pub information: Vec<cct::Text>,
/// An indicator that this piece of transport equipment is returnable (true) or not (false).
    #[serde(default, rename = "ReturnabilityIndicator")]
    pub returnability_indicator: Option<udt::Indicator>,
/// An indication of the legal status of this piece of transport equipment with respect to the Container
/// Convention Code.
    #[serde(default, rename = "LegalStatusIndicator")]
    pub legal_status_indicator: Option<udt::Indicator>,
/// The percent of the airflow within this piece of transport equipment.
    #[serde(default, rename = "AirFlowPercent")]
    pub air_flow_percent: Option<cct::Numeric>,
/// The percent humidity within this piece of transport equipment.
    #[serde(default, rename = "HumidityPercent")]
    pub humidity_percent: Option<cct::Numeric>,
/// An indicator that this piece of transport equipment is approved for animal food (true) or not
/// (false).
    #[serde(default, rename = "AnimalFoodApprovedIndicator")]
    pub animal_food_approved_indicator: Option<udt::Indicator>,
/// An indicator that this piece of transport equipment is approved for human food (true) or not
/// (false).
    #[serde(default, rename = "HumanFoodApprovedIndicator")]
    pub human_food_approved_indicator: Option<udt::Indicator>,
/// An indicator that this piece of transport equipment is approved for dangerous goods (true) or not
/// (false).
    #[serde(default, rename = "DangerousGoodsApprovedIndicator")]
    pub dangerous_goods_approved_indicator: Option<udt::Indicator>,
/// An indicator that this piece of transport equipment is refrigerated (true) or not (false).
    #[serde(default, rename = "RefrigeratedIndicator")]
    pub refrigerated_indicator: Option<udt::Indicator>,
/// Characteristics of this piece of transport equipment.
    #[serde(default, rename = "Characteristics")]
    pub characteristics: Option<cct::Text>,
/// Damage associated with this piece of transport equipment.
    #[serde(default, rename = "DamageRemarks")]
    pub damage_remarks: Vec<cct::Text>,
/// Text describing this piece of transport equipment.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Special transport requirements expressed as text.
    #[serde(default, rename = "SpecialTransportRequirements")]
    pub special_transport_requirements: Vec<cct::Text>,
/// The gross weight of this piece of transport equipment.
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: Option<cct::Measure>,
/// The gross volume of this piece of transport equipment.
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: Option<cct::Measure>,
/// The weight of this piece of transport equipment when empty.
    #[serde(default, rename = "TareWeightMeasure")]
    pub tare_weight_measure: Option<cct::Measure>,
/// A code signifying the tracking device for this piece of transport equipment.
    #[serde(default, rename = "TrackingDeviceCode")]
    pub tracking_device_code: Option<cct::Code>,
/// An indicator that this piece of transport equipment can supply power (true) or not (false).
    #[serde(default, rename = "PowerIndicator")]
    pub power_indicator: Option<udt::Indicator>,
/// An identifier for use in tracing this piece of transport equipment, such as the EPC number used in
/// RFID.
    #[serde(default, rename = "TraceID")]
    pub trace_id: Option<cct::Identifier>,
/// The Stowage Position identifier for this piece of carried logistics Transport Equipment.
    #[serde(default, rename = "StowagePositionID")]
    pub stowage_position_id: Option<cct::Identifier>,
/// A measurable dimension (length, mass, weight, or volume) of this piece of transport equipment.
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: Vec<crate::Dimension>,
/// A seal securing the door of a piece of transport equipment.
    #[serde(default, rename = "TransportEquipmentSeal")]
    pub transport_equipment_seal: Vec<TransportEquipmentSeal>,
/// In the case of a refrigeration unit, the minimum allowable operating temperature for this container.
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<crate::Temperature>,
/// In the case of a refrigeration unit, the maximum allowable operating temperature for this container.
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<crate::Temperature>,
/// The Party who provides this piece of Transport Equipment.
    #[serde(default, rename = "ProviderParty")]
    pub provider_party: Option<crate::Party>,
/// The authorised Party who certifies the load of the Goods into this piece of Transport Equipment.
    #[serde(default, rename = "LoadingProofParty")]
    pub loading_proof_party: Option<crate::Party>,
/// The party that supplies this piece of transport equipment.
    #[serde(default, rename = "SupplierParty")]
    pub supplier_party: Option<crate::SupplierParty>,
/// The Party who owns this Piece of Transport Equipment.
    #[serde(default, rename = "OwnerParty")]
    pub owner_party: Option<crate::Party>,
/// The Party who operates this piece of Transport Equipment.
    #[serde(default, rename = "OperatingParty")]
    pub operating_party: Option<crate::Party>,
/// The location where this piece of transport equipment is loaded.
    #[serde(default, rename = "LoadingLocation")]
    pub loading_location: Option<crate::Location>,
/// The location where this piece of transport equipment is unloaded.
    #[serde(default, rename = "UnloadingLocation")]
    pub unloading_location: Option<crate::Location>,
/// The location where this piece of transport equipment is being stored.
    #[serde(default, rename = "StorageLocation")]
    pub storage_location: Option<crate::Location>,
/// A positioning of this piece of transport equipment.
    #[serde(default, rename = "PositioningTransportEvent")]
    pub positioning_transport_event: Vec<TransportEvent>,
/// A quarantine of this piece of transport equipment.
    #[serde(default, rename = "QuarantineTransportEvent")]
    pub quarantine_transport_event: Vec<TransportEvent>,
/// A delivery of this piece of transport equipment.
    #[serde(default, rename = "DeliveryTransportEvent")]
    pub delivery_transport_event: Vec<TransportEvent>,
/// A pickup of this piece of transport equipment.
    #[serde(default, rename = "PickupTransportEvent")]
    pub pickup_transport_event: Vec<TransportEvent>,
/// A handling of this piece of transport equipment.
    #[serde(default, rename = "HandlingTransportEvent")]
    pub handling_transport_event: Vec<TransportEvent>,
/// A loading of this piece of transport equipment.
    #[serde(default, rename = "LoadingTransportEvent")]
    pub loading_transport_event: Vec<TransportEvent>,
/// An additional transport event not specified elsewhere in this Transport Equipment.
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: Vec<TransportEvent>,
/// The applicable transport means associated with this piece of transport equipment.
    #[serde(default, rename = "ApplicableTransportMeans")]
    pub applicable_transport_means: Option<TransportMeans>,
/// A set of haulage trading terms associated with this piece of transport equipment.
    #[serde(default, rename = "HaulageTradingTerms")]
    pub haulage_trading_terms: Vec<crate::TradingTerms>,
/// Transit-related information regarding a type of hazardous goods contained in this piece of transport
/// equipment.
    #[serde(default, rename = "HazardousGoodsTransit")]
    pub hazardous_goods_transit: Vec<crate::HazardousGoodsTransit>,
/// A packaged transport handling unit associated with this piece of transport equipment.
    #[serde(default, rename = "PackagedTransportHandlingUnit")]
    pub packaged_transport_handling_unit: Vec<TransportHandlingUnit>,
/// A service allowance charge associated with this piece of transport equipment.
    #[serde(default, rename = "ServiceAllowanceCharge")]
    pub service_allowance_charge: Vec<crate::AllowanceCharge>,
/// A freight allowance charge associated with this piece of transport equipment.
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: Vec<crate::AllowanceCharge>,
/// A piece of transport equipment attached to this piece of transport equipment.
    #[serde(default, rename = "AttachedTransportEquipment")]
    pub attached_transport_equipment: Vec<TransportEquipment>,
/// The delivery of this piece of transport equipment.
    #[serde(default, rename = "Delivery")]
    pub delivery: Option<Box<crate::Delivery>>,
/// The pickup of this piece of transport equipment.
    #[serde(default, rename = "Pickup")]
    pub pickup: Option<crate::Pickup>,
/// The despatch of this piece of transport equipment.
    #[serde(default, rename = "Despatch")]
    pub despatch: Option<crate::Despatch>,
/// A reference to a shipping document associated with this piece of transport equipment.
    #[serde(default, rename = "ShipmentDocumentReference")]
    pub shipment_document_reference: Vec<crate::DocumentReference>,
/// A piece of transport equipment contained in this piece of transport equipment.
    #[serde(default, rename = "ContainedInTransportEquipment")]
    pub contained_in_transport_equipment: Vec<TransportEquipment>,
/// A package contained in this piece of transport equipment.
    #[serde(default, rename = "Package")]
    pub package: Vec<crate::Package>,
/// A goods item contained in this piece of transport equipment.
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Vec<crate::GoodsItem>,
/// The verified gross mass of this piece of transport equipment.
    #[serde(default, rename = "VerifiedGrossMass")]
    pub verified_gross_mass: Option<crate::VerifiedGrossMass>,
/// Hazardous items loaded into this transport equipment
    #[serde(default, rename = "LoadedHazardousItem")]
    pub loaded_hazardous_item: Vec<crate::HazardousItem>,
}
