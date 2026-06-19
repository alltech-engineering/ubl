#[derive(Debug, Deserialize, Serialize)]
/// A document used in the negotiation of a transport service between a transport user and a transport
/// service provider.
///
/// UBL Dictionary Entry Name: `Transport Execution Plan. Details`
///
/// Generated from XSD type `TransportExecutionPlanType`.
pub struct TransportExecutionPlan {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
/// Identifies the earliest version of the UBL 2 schema for this document type that defines all of the
/// elements that might be encountered in the current instance.
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
/// Identifies a user-defined customization of UBL for a specific use.
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
/// Identifies a user-defined profile of the customization of UBL being used.
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
/// Identifies an instance of executing a profile, to associate all transactions in a collaboration.
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
/// An identifier for this document, assigned by the sender.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// Indicates the current version of the Transport Execution Plan.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// (Deprecated) Indicates whether this document is a copy (true) or not (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// A code signifying the status of the Transport Execution Plan (updated, cancelled, confirmed, etc.)
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<cct::Code>,
/// A code signifying a reason associated with the status of a Transport Execution Plan.
    #[serde(default, rename = "DocumentStatusReasonCode")]
    pub document_status_reason_code: Option<cct::Code>,
/// A reason for the status assigned to the Transport Execution Plan, expressed in text.
    #[serde(default, rename = "DocumentStatusReasonDescription")]
    pub document_status_reason_description: Vec<cct::Text>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Remarks from the transport user regarding the transport operations referred to in the Transport
/// Execution Plan.
    #[serde(default, rename = "TransportUserRemarks")]
    pub transport_user_remarks: Vec<cct::Text>,
/// Remarks from the transport service provider regarding the transport operations referred to in the
/// Transport Execution Plan.
    #[serde(default, rename = "TransportServiceProviderRemarks")]
    pub transport_service_provider_remarks: Vec<cct::Text>,
/// The Party who sends this Document. This Party is normally the transport user or the Transport
/// Service Provider.
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
/// The Party who receives this Document. This Party is normally the transport user or the Transport
/// Service Provider.
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
/// The Party who requests the transport service from a Transport Service Provider.
    #[serde(default, rename = "TransportUserParty")]
    pub transport_user_party: Option<cac::Party>,
/// The Party who offers the transport service based upon a request from a transport user.
    #[serde(rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: cac::Party,
/// The Party who executes the Payment for the transport service provided in the Transport Execution
/// Plan.
    #[serde(default, rename = "BillToParty")]
    pub bill_to_party: Option<cac::Party>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// A reference to a Transport Execution Plan Request.
    #[serde(default, rename = "TransportExecutionPlanRequestDocumentReference")]
    pub transport_execution_plan_request_document_reference:
        Option<cac::DocumentReference>,
/// A reference to an original Transport Execution Plan.
    #[serde(default, rename = "TransportExecutionPlanDocumentReference")]
    pub transport_execution_plan_document_reference:
        Option<cac::DocumentReference>,
/// A reference to the Transport Service Description, which is used by a transport service provider to
/// announce transport services to transport users (buyers).
    #[serde(default, rename = "TransportServiceDescriptionDocumentReference")]
    pub transport_service_description_document_reference:
        Option<cac::DocumentReference>,
/// A reference to an additional document associated with this document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// A contract related to the Transport Execution Plan.
    #[serde(default, rename = "TransportContract")]
    pub transport_contract: Option<cac::Contract>,
/// Describes the deadline for when the Transport Service Provider will have to respond to a Transport
/// Execution Plan .
    #[serde(default, rename = "TransportServiceProviderResponseRequiredPeriod")]
    pub transport_service_provider_response_required_period:
        Option<cac::Period>,
/// Describes the deadline for when the Transport User will have to respond to a Transport Execution
/// Plan suggested by a Transport Service Provider.
    #[serde(default, rename = "TransportUserResponseRequiredPeriod")]
    pub transport_user_response_required_period: Vec<cac::Period>,
/// A period during which the Transport Execution Plan is valid.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<cac::Period>,
/// Description of the main transportation service referenced in the Transport Execution Plan.
    #[serde(default, rename = "MainTransportationService")]
    pub main_transportation_service: Option<cac::TransportationService>,
/// A description of an additional transportation service referenced in the Transport Execution Plan.
    #[serde(default, rename = "AdditionalTransportationService")]
    pub additional_transportation_service: Vec<cac::TransportationService>,
/// The period within which the service must begin.
    #[serde(default, rename = "ServiceStartTimePeriod")]
    pub service_start_time_period: Option<cac::Period>,
/// The period during which the service must be completed.
    #[serde(default, rename = "ServiceEndTimePeriod")]
    pub service_end_time_period: Option<cac::Period>,
/// The location of origin of the transport service referenced in the Transport Execution Plan.
    #[serde(default, rename = "FromLocation")]
    pub from_location: Option<cac::Location>,
/// The destination location for the transport service referenced in the Transport Execution Plan.
    #[serde(default, rename = "ToLocation")]
    pub to_location: Option<cac::Location>,
/// The location of a transport service (e.g., terminal handling service) that does not require
/// transport movement.
    #[serde(default, rename = "AtLocation")]
    pub at_location: Option<cac::Location>,
/// A description of terms and conditions related to the Transport Execution Plan.
    #[serde(default, rename = "TransportExecutionTerms")]
    pub transport_execution_terms: Option<cac::TransportExecutionTerms>,
/// A description of an identifiable collection of goods items to be transported between the consignor
/// and the consignee. This information may be defined within a transport contract. A consignment may
/// comprise more than one shipment (e.g., when consolidated by a freight forwarder).
    #[serde(default, rename = "Consignment")]
    pub consignment: Vec<cac::Consignment>,
}
