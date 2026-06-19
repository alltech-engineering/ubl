#[derive(Debug, Deserialize, Serialize)]
pub struct TransportExecutionPlan {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<cct::Code>,
    #[serde(default, rename = "DocumentStatusReasonCode")]
    pub document_status_reason_code: Option<cct::Code>,
    #[serde(default, rename = "DocumentStatusReasonDescription")]
    pub document_status_reason_description: Vec<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "TransportUserRemarks")]
    pub transport_user_remarks: Vec<cct::Text>,
    #[serde(default, rename = "TransportServiceProviderRemarks")]
    pub transport_service_provider_remarks: Vec<cct::Text>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
    #[serde(default, rename = "TransportUserParty")]
    pub transport_user_party: Option<cac::Party>,
    #[serde(rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: cac::Party,
    #[serde(default, rename = "BillToParty")]
    pub bill_to_party: Option<cac::Party>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "TransportExecutionPlanRequestDocumentReference")]
    pub transport_execution_plan_request_document_reference:
        Option<cac::DocumentReference>,
    #[serde(default, rename = "TransportExecutionPlanDocumentReference")]
    pub transport_execution_plan_document_reference:
        Option<cac::DocumentReference>,
    #[serde(default, rename = "TransportServiceDescriptionDocumentReference")]
    pub transport_service_description_document_reference:
        Option<cac::DocumentReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "TransportContract")]
    pub transport_contract: Option<cac::Contract>,
    #[serde(default, rename = "TransportServiceProviderResponseRequiredPeriod")]
    pub transport_service_provider_response_required_period:
        Option<cac::Period>,
    #[serde(default, rename = "TransportUserResponseRequiredPeriod")]
    pub transport_user_response_required_period: Vec<cac::Period>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<cac::Period>,
    #[serde(default, rename = "MainTransportationService")]
    pub main_transportation_service: Option<cac::TransportationService>,
    #[serde(default, rename = "AdditionalTransportationService")]
    pub additional_transportation_service: Vec<cac::TransportationService>,
    #[serde(default, rename = "ServiceStartTimePeriod")]
    pub service_start_time_period: Option<cac::Period>,
    #[serde(default, rename = "ServiceEndTimePeriod")]
    pub service_end_time_period: Option<cac::Period>,
    #[serde(default, rename = "FromLocation")]
    pub from_location: Option<cac::Location>,
    #[serde(default, rename = "ToLocation")]
    pub to_location: Option<cac::Location>,
    #[serde(default, rename = "AtLocation")]
    pub at_location: Option<cac::Location>,
    #[serde(default, rename = "TransportExecutionTerms")]
    pub transport_execution_terms: Option<cac::TransportExecutionTerms>,
    #[serde(default, rename = "Consignment")]
    pub consignment: Vec<cac::Consignment>,
}
