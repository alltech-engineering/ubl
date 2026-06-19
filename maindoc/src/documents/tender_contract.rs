#[derive(Debug, Deserialize, Serialize)]
/// A document published by a Contracting Party to announce the awarding of a procurement project.
///
/// UBL Dictionary Entry Name: `Tender Contract. Details`
///
/// Generated from XSD type `TenderContractType`.
pub struct TenderContract {
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
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// (Deprecated) Indicates whether this document is a copy (true) or not (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// An identifier, assigned by the sender, for the process file (i.e., record) to which this document
/// belongs.
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::Identifier,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Short title of a contract associated with this Tender.
    #[serde(default, rename = "ContractName")]
    pub contract_name: Vec<cct::Text>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Information about the law that defines the regulatory domain.
    #[serde(default, rename = "RegulatoryDomain")]
    pub regulatory_domain: Vec<cct::Text>,
/// An indicator specifying if the notice is published for service contracts within certain service
/// categories (true) or not (false).
    #[serde(default, rename = "PublishAwardIndicator")]
    pub publish_award_indicator: Option<udt::Indicator>,
/// A reference to a previously sent document.
    #[serde(default, rename = "PreviousDocumentReference")]
    pub previous_document_reference: Vec<cac::DocumentReference>,
/// A reference to a set of minutes.
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The contracting party.
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: Vec<cac::ContractingParty>,
/// The Economic Operator issuing the inquiry on the status of a tendering process.
    #[serde(default, rename = "EconomicOperatorParty")]
    pub economic_operator_party: Vec<cac::EconomicOperatorParty>,
/// The Party who receives this Document.
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
/// The tendering terms associated with this tendering process.
    #[serde(default, rename = "TenderingTerms")]
    pub tendering_terms: Option<cac::TenderingTerms>,
/// A description of the tendering process itself.
    #[serde(default, rename = "TenderingProcess")]
    pub tendering_process: Option<cac::TenderingProcess>,
/// An overall definition of this procurement project.
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: Option<cac::ProcurementProject>,
/// Lots that were awarded to the economic operator that can be grouped in the same contract.
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Vec<cac::ProcurementProjectLot>,
/// A result of the bid opening in the tendering process.
    #[serde(default, rename = "TenderResult")]
    pub tender_result: Vec<cac::TenderResult>,
}
