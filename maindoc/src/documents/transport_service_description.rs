#[derive(Debug, Deserialize, Serialize)]
pub struct TransportServiceDescription {
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
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "ServiceName")]
    pub service_name: Option<cct::Text>,
    #[serde(default, rename = "ResponseCode")]
    pub response_code: Option<cct::Code>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
    #[serde(
        default,
        rename = "TransportServiceDescriptionRequestDocumentReference"
    )]
    pub transport_service_description_request_document_reference:
        Option<cac::DocumentReference>,
    #[serde(default, rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: Option<cac::Party>,
    #[serde(default, rename = "ServiceChargePaymentTerms")]
    pub service_charge_payment_terms: Option<cac::PaymentTerms>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<cac::Period>,
    #[serde(default, rename = "TransportationService")]
    pub transportation_service: Vec<cac::TransportationService>,
}
