#[derive(Debug, Deserialize, Serialize)]
/// A document requesting a Transport Service Description, sent from a party with a transport demand
/// (transport user) to a party providing transport services (transport service provider).
///
/// UBL Dictionary Entry Name: `Transport Service Description Request. Details`
///
/// Generated from XSD type `TransportServiceDescriptionRequestType`.
pub struct TransportServiceDescriptionRequest {
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
/// (Deprecated) Indicates whether this document is a copy (true) or not (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(rename = "IssueTime")]
    pub issue_time: udt::DateTime,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the category of service information requested to be provided in the Transport
/// Service Description.
    #[serde(default, rename = "ServiceInformationPreferenceCode")]
    pub service_information_preference_code: Option<cct::Code>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who sends the Transport Service Description Request.
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
/// The Party who receives the Transport Service Description Request.
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
/// The Transport Service Provider.
    #[serde(default, rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: Option<cac::Party>,
/// A transportation service about which information is requested.
    #[serde(default, rename = "TransportationService")]
    pub transportation_service: Vec<cac::TransportationService>,
}
