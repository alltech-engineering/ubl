#[derive(Debug, Deserialize, Serialize)]
pub struct TransportExecutionPlanRequest {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<cct::CodeType>,
    #[serde(default, rename = "DocumentStatusReasonCode")]
    pub document_status_reason_code: Option<cct::CodeType>,
    #[serde(default, rename = "DocumentStatusReasonDescription")]
    pub document_status_reason_description: Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "TransportUserRemarks")]
    pub transport_user_remarks: Vec<cct::TextType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
    #[serde(default, rename = "TransportUserParty")]
    pub transport_user_party: Option<cac::Party>,
    #[serde(rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: cac::Party,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: Option<cac::Party>,
    #[serde(default, rename = "BillToParty")]
    pub bill_to_party: Option<cac::Party>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
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
    #[serde(default, rename = "TransportServiceProviderResponseDeadlinePeriod")]
    pub transport_service_provider_response_deadline_period: Vec<cac::Period>,
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
