use serde::{Deserialize, Serialize};

include!("specification.rs");
include!("event_line_item.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct PromotionalEvent {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "PromotionalEventTypeCode")]
    pub promotional_event_type_code: cct::Code,
    #[serde(default, rename = "SubmissionDate")]
    pub submission_date: Option<udt::DateTime>,
    #[serde(default, rename = "FirstShipmentAvailibilityDate")]
    pub first_shipment_availibility_date: Option<udt::DateTime>,
    #[serde(default, rename = "FirstShipmentAvailabilityDate")]
    pub first_shipment_availability_date: Option<udt::DateTime>,
    #[serde(default, rename = "LatestProposalAcceptanceDate")]
    pub latest_proposal_acceptance_date: Option<udt::DateTime>,
    #[serde(default, rename = "PromotionalSpecification")]
    pub promotional_specification: Vec<PromotionalSpecification>,
}
