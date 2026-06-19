use serde::{Deserialize, Serialize};


include!("service.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe one segment or leg in a transportation service.
///
/// UBL Dictionary Entry Name: `Transportation Segment. Details`
///
/// Generated from XSD type `TransportationSegmentType`.
pub struct TransportationSegment {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A number indicating the order of this segment in the sequence of transportation segments making up a
/// transportation service.
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: cct::Numeric,
/// An identifier for the transport execution plan governing this transportation segment.
    #[serde(default, rename = "TransportExecutionPlanReferenceID")]
    pub transport_execution_plan_reference_id: Option<cct::Identifier>,
/// The transportation service used in this transportation segment.
    #[serde(rename = "TransportationService")]
    pub transportation_service: TransportationService,
/// The Transport Service Provider who is reponsible for the Transportation Service in this
/// Transportation Segment.
    #[serde(rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: crate::Party,
/// A consignment referenced in this transportation segment. Such a consignment may have different
/// identifiers than the consignment identifiers being used in the transportation service agreed between
/// the transport user and the transport service provider.
    #[serde(default, rename = "ReferencedConsignment")]
    pub referenced_consignment: Option<crate::Consignment>,
/// The shipment stage associated with this transportation segment.
    #[serde(default, rename = "ShipmentStage")]
    pub shipment_stage: Vec<crate::ShipmentStage>,
}
