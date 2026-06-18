#[derive(Debug, Deserialize, Serialize)]
pub struct TransportationSegment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: super::cct::NumericType,
    #[serde(default, rename = "TransportExecutionPlanReferenceID")]
    pub transport_execution_plan_reference_id: Option<super::cct::IdentifierType>,
    #[serde(rename = "TransportationService")]
    pub transportation_service: TransportationService,
    #[serde(rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: Party,
    #[serde(default, rename = "ReferencedConsignment")]
    pub referenced_consignment: Option<Consignment>,
    #[serde(default, rename = "ShipmentStage")]
    pub shipment_stage: Vec<ShipmentStage>,
}
