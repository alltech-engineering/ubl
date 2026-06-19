#[derive(Debug, Deserialize, Serialize)]
pub struct TransportationService {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "TransportServiceCode")]
    pub transport_service_code: cct::Code,
    #[serde(default, rename = "TariffClassCode")]
    pub tariff_class_code: Option<cct::Code>,
    #[serde(default, rename = "Priority")]
    pub priority: Option<cct::Text>,
    #[serde(default, rename = "FreightRateClassCode")]
    pub freight_rate_class_code: Option<cct::Code>,
    #[serde(default, rename = "TransportationServiceDescription")]
    pub transportation_service_description: Vec<cct::Text>,
    #[serde(default, rename = "TransportationServiceDetailsURI")]
    pub transportation_service_details_uri: Option<cct::Identifier>,
    #[serde(default, rename = "NominationDate")]
    pub nomination_date: Option<udt::DateTime>,
    #[serde(default, rename = "NominationTime")]
    pub nomination_time: Option<udt::DateTime>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "SequenceNumeric")]
    pub sequence_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "TransportEquipment")]
    pub transport_equipment: Vec<crate::TransportEquipment>,
    #[serde(default, rename = "SupportedTransportEquipment")]
    pub supported_transport_equipment: Vec<crate::TransportEquipment>,
    #[serde(default, rename = "UnsupportedTransportEquipment")]
    pub unsupported_transport_equipment: Vec<crate::TransportEquipment>,
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: Vec<crate::CommodityClassification>,
    #[serde(default, rename = "SupportedCommodityClassification")]
    pub supported_commodity_classification: Vec<crate::CommodityClassification>,
    #[serde(default, rename = "UnsupportedCommodityClassification")]
    pub unsupported_commodity_classification: Vec<crate::CommodityClassification>,
    #[serde(default, rename = "TotalCapacityDimension")]
    pub total_capacity_dimension: Option<crate::Dimension>,
    #[serde(default, rename = "ShipmentStage")]
    pub shipment_stage: Vec<crate::ShipmentStage>,
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: Vec<crate::TransportEvent>,
    #[serde(default, rename = "ResponsibleTransportServiceProviderParty")]
    pub responsible_transport_service_provider_party: Option<crate::Party>,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<crate::EnvironmentalEmission>,
    #[serde(default, rename = "EstimatedDurationPeriod")]
    pub estimated_duration_period: Option<crate::Period>,
    #[serde(default, rename = "ScheduledServiceFrequency")]
    pub scheduled_service_frequency: Vec<crate::ServiceFrequency>,
}
