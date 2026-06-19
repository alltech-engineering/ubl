use serde::{Deserialize, Serialize};


include!("service.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct TransportationSegment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: cct::Numeric,
    #[serde(default, rename = "TransportExecutionPlanReferenceID")]
    pub transport_execution_plan_reference_id: Option<cct::Identifier>,
    #[serde(rename = "TransportationService")]
    pub transportation_service: TransportationService,
    #[serde(rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: crate::Party,
    #[serde(default, rename = "ReferencedConsignment")]
    pub referenced_consignment: Option<crate::Consignment>,
    #[serde(default, rename = "ShipmentStage")]
    pub shipment_stage: Vec<crate::ShipmentStage>,
}
