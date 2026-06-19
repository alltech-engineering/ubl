use serde::{Deserialize, Serialize};

include!("specification.rs");
include!("event_line_item.rs");

#[derive(Debug, Deserialize, Serialize)]
/// Agree can be renamed as PromotionalEvents
///
/// UBL Dictionary Entry Name: `Promotional Event. Details`
///
/// Generated from XSD type `PromotionalEventType`.
pub struct PromotionalEvent {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code signifying the type of this promotional event. Examples can be: Holiday, Seasonal Event,
/// Store Closing, Trade Item Introduction
    #[serde(rename = "PromotionalEventTypeCode")]
    pub promotional_event_type_code: cct::Code,
/// The date on which a proposal for this promotional event was submitted.
    #[serde(default, rename = "SubmissionDate")]
    pub submission_date: Option<udt::DateTime>,
/// (Deprecated) The first day that products will be available to ship from buyer to seller if the
/// proposal for this promotional event is accepted.
    #[serde(default, rename = "FirstShipmentAvailibilityDate")]
    pub first_shipment_availibility_date: Option<udt::DateTime>,
/// The first day that products will be available to ship from buyer to seller if the proposal for this
/// promotional event is accepted.
    #[serde(default, rename = "FirstShipmentAvailabilityDate")]
    pub first_shipment_availability_date: Option<udt::DateTime>,
/// The deadline for acceptance of this promotional event.
    #[serde(default, rename = "LatestProposalAcceptanceDate")]
    pub latest_proposal_acceptance_date: Option<udt::DateTime>,
/// A specification for a promotional event.
    #[serde(default, rename = "PromotionalSpecification")]
    pub promotional_specification: Vec<PromotionalSpecification>,
}
