#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a transportation service.
///
/// UBL Dictionary Entry Name: `Transportation Service. Details`
///
/// Generated from XSD type `TransportationServiceType`.
pub struct TransportationService {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A code signifying the extent of this transportation service (e.g., door-to-door, port-to-port).
    #[serde(rename = "TransportServiceCode")]
    pub transport_service_code: cct::Code,
/// A code signifying the tariff class applicable to this transportation service.
    #[serde(default, rename = "TariffClassCode")]
    pub tariff_class_code: Option<cct::Code>,
/// The priority of this transportation service.
    #[serde(default, rename = "Priority")]
    pub priority: Option<cct::Text>,
/// A code signifying the rate class for freight in this transportation service.
    #[serde(default, rename = "FreightRateClassCode")]
    pub freight_rate_class_code: Option<cct::Code>,
/// Text describing this transportation service.
    #[serde(default, rename = "TransportationServiceDescription")]
    pub transportation_service_description: Vec<cct::Text>,
/// The Uniform Resource Identifier (URI) of a document providing additional details regarding this
/// transportation service.
    #[serde(default, rename = "TransportationServiceDetailsURI")]
    pub transportation_service_details_uri: Option<cct::Identifier>,
/// In a transport contract, the deadline date by which this transportation service has to be booked.
/// For example, if this service is scheduled for Wednesday 16 February 2011 at 10 a.m. CET, the
/// nomination date might be Tuesday15 February 2011.
    #[serde(default, rename = "NominationDate")]
    pub nomination_date: Option<udt::DateTime>,
/// In a transport contract, the deadline time by which this transportation service has to be booked.
/// For example, if this service is scheduled for Wednesday 16 February 2011 at 10 a.m. CET, the
/// nomination date might be Tuesday15 February 2011 and the nomination time 4 p.m. at the latest.
    #[serde(default, rename = "NominationTime")]
    pub nomination_time: Option<udt::DateTime>,
/// The name of this transportation service.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// A number indicating the order of this transportation service in a sequence of transportation
/// services.
    #[serde(default, rename = "SequenceNumeric")]
    pub sequence_numeric: Option<cct::Numeric>,
/// A piece of transport equipment used in this transportation service.
    #[serde(default, rename = "TransportEquipment")]
    pub transport_equipment: Vec<crate::TransportEquipment>,
/// A piece of transport equipment supported in this transportation service.
    #[serde(default, rename = "SupportedTransportEquipment")]
    pub supported_transport_equipment: Vec<crate::TransportEquipment>,
/// A piece of transport equipment that is not supported in this transportation service.
    #[serde(default, rename = "UnsupportedTransportEquipment")]
    pub unsupported_transport_equipment: Vec<crate::TransportEquipment>,
/// A classification of this transportation service.
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: Vec<crate::CommodityClassification>,
/// A classification (e.g., general cargo) for commodities that can be handled in this transportation
/// service.
    #[serde(default, rename = "SupportedCommodityClassification")]
    pub supported_commodity_classification: Vec<crate::CommodityClassification>,
/// A classification for commodities that cannot be handled in this transportation service.
    #[serde(default, rename = "UnsupportedCommodityClassification")]
    pub unsupported_commodity_classification: Vec<crate::CommodityClassification>,
/// The total capacity or volume available in this transportation service.
    #[serde(default, rename = "TotalCapacityDimension")]
    pub total_capacity_dimension: Option<crate::Dimension>,
/// One or more stages of shipment in this transportation service.
    #[serde(default, rename = "ShipmentStage")]
    pub shipment_stage: Vec<crate::ShipmentStage>,
/// One or more transport events taking place in this transportation service.
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: Vec<crate::TransportEvent>,
/// The Transport Service Provider who is reponsible for this Transportation Service.
    #[serde(default, rename = "ResponsibleTransportServiceProviderParty")]
    pub responsible_transport_service_provider_party: Option<crate::Party>,
/// An environmental emission resulting from this transportation service.
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<crate::EnvironmentalEmission>,
/// The estimated duration of this transportation service.
    #[serde(default, rename = "EstimatedDurationPeriod")]
    pub estimated_duration_period: Option<crate::Period>,
/// A class to specify which day of the week a transport service is operational.
    #[serde(default, rename = "ScheduledServiceFrequency")]
    pub scheduled_service_frequency: Vec<crate::ServiceFrequency>,
}
