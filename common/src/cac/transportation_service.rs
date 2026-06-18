#[derive(Debug, Deserialize, Serialize)]
pub struct TransportationService {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "TransportServiceCode")]
    pub transport_service_code: super::cct::CodeType,
    #[serde(default, rename = "TariffClassCode")]
    pub tariff_class_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Priority")]
    pub priority: Option<super::cct::TextType>,
    #[serde(default, rename = "FreightRateClassCode")]
    pub freight_rate_class_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TransportationServiceDescription")]
    pub transportation_service_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "TransportationServiceDetailsURI")]
    pub transportation_service_details_uri: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "NominationDate")]
    pub nomination_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "NominationTime")]
    pub nomination_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "SequenceNumeric")]
    pub sequence_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "TransportEquipment")]
    pub transport_equipment: Vec<TransportEquipment>,
    #[serde(default, rename = "SupportedTransportEquipment")]
    pub supported_transport_equipment: Vec<TransportEquipment>,
    #[serde(default, rename = "UnsupportedTransportEquipment")]
    pub unsupported_transport_equipment: Vec<TransportEquipment>,
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: Vec<CommodityClassification>,
    #[serde(default, rename = "SupportedCommodityClassification")]
    pub supported_commodity_classification: Vec<CommodityClassification>,
    #[serde(default, rename = "UnsupportedCommodityClassification")]
    pub unsupported_commodity_classification: Vec<CommodityClassification>,
    #[serde(default, rename = "TotalCapacityDimension")]
    pub total_capacity_dimension: Option<Dimension>,
    #[serde(default, rename = "ShipmentStage")]
    pub shipment_stage: Vec<ShipmentStage>,
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: Vec<TransportEvent>,
    #[serde(default, rename = "ResponsibleTransportServiceProviderParty")]
    pub responsible_transport_service_provider_party: Option<Party>,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<EnvironmentalEmission>,
    #[serde(default, rename = "EstimatedDurationPeriod")]
    pub estimated_duration_period: Option<Period>,
    #[serde(default, rename = "ScheduledServiceFrequency")]
    pub scheduled_service_frequency: Vec<ServiceFrequency>,
}
