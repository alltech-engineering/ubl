#[derive(Debug, Deserialize, Serialize)]
/// A document listing the contents, cargo, passengers and crew of an airplane, a ship, a truck or a
/// wagon.
///
/// UBL Dictionary Entry Name: `Manifest. Details`
///
/// Generated from XSD type `ManifestType`.
pub struct Manifest {
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
/// An identifier for this document.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// The type of Manifest, expressed as a code.
    #[serde(default, rename = "ManifestTypeCode")]
    pub manifest_type_code: Option<cct::Code>,
/// The type of Manifest, expressed as text.
    #[serde(default, rename = "ManifestType")]
    pub manifest_type: Vec<cct::Text>,
/// Textual description of this document instance.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies a version of a common transportation report in order to distinguish updates.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// An indicator of whether ad valorem duties are levied on commodities described in this manifest
/// (true) or not (false).
    #[serde(default, rename = "AdValoremIndicator")]
    pub ad_valorem_indicator: Option<udt::Indicator>,
/// Value declared by the shipper or his agent for the purpose of varying the carrier's level of
/// liability from that provided in the contract of carriage in case of loss or damage to goods or
/// delayed delivery.
    #[serde(default, rename = "DeclaredCarriageValueAmount")]
    pub declared_carriage_value_amount: Option<cct::Amount>,
/// The Party who issues this Manifest.This Party is normally the Logistics Operator.
    #[serde(rename = "SendingLogisticsOperatorParty")]
    pub sending_logistics_operator_party: cac::Party,
/// The Authority or regulator who receives this Manifest.
    #[serde(default, rename = "AuthorityParty")]
    pub authority_party: Option<cac::Party>,
/// The Party who is reponsible for sending the goods.
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: Option<cac::Party>,
/// The Party who receives the goods.
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: Option<cac::Party>,
/// A person registred as crew in this manifest.
    #[serde(default, rename = "CrewPerson")]
    pub crew_person: Vec<cac::Person>,
/// A person registred as passenger in this manifest
    #[serde(default, rename = "PassengerPerson")]
    pub passenger_person: Vec<cac::Person>,
/// A shipment associated with this manifest.
    #[serde(default, rename = "Shipment")]
    pub shipment: Option<cac::Shipment>,
/// A reference to a document relevant for or associated with this common transportation report.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
/// An interested party to whom this document is distributed.
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: Vec<cac::DocumentDistribution>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
