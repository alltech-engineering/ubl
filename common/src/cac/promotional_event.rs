#[derive(Debug, Deserialize, Serialize)]
pub struct PromotionalEvent {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "PromotionalEventTypeCode")]
    pub promotional_event_type_code: super::cct::CodeType,
    #[serde(default, rename = "SubmissionDate")]
    pub submission_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "FirstShipmentAvailibilityDate")]
    pub first_shipment_availibility_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "FirstShipmentAvailabilityDate")]
    pub first_shipment_availability_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "LatestProposalAcceptanceDate")]
    pub latest_proposal_acceptance_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "PromotionalSpecification")]
    pub promotional_specification: Vec<PromotionalSpecification>,
}
