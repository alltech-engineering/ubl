#[derive(Debug, Deserialize, Serialize)]
/// A document used by a Contracting Party to define a procurement project to buy goods, services, or
/// works during a specified period.
///
/// UBL Dictionary Entry Name: `Call For Tenders. Details`
///
/// Generated from XSD type `CallForTendersType`.
pub struct CallForTenders {
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
/// The date, assigned by the contracting party, on which the Call For Tenders was approved.
    #[serde(default, rename = "ApprovalDate")]
    pub approval_date: Option<udt::DateTime>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Indicates the current version of the Call for Tenders.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// Identifies the previous version of the Call for Tenders which is superceded by this version.
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<cct::Identifier>,
/// A reference to a legal document.
    #[serde(default, rename = "LegalDocumentReference")]
    pub legal_document_reference: Option<cac::DocumentReference>,
/// A reference to a technical document.
    #[serde(default, rename = "TechnicalDocumentReference")]
    pub technical_document_reference: Option<cac::DocumentReference>,
/// A reference to a required document.
    #[serde(default, rename = "RequiredDocumentReference")]
    pub required_document_reference: Vec<cac::DocumentReference>,
/// A reference to a provided document.
    #[serde(default, rename = "ProvidedDocumentReference")]
    pub provided_document_reference: Vec<cac::DocumentReference>,
/// A reference to an additional document associated with this document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The contracting party or parties in case of joint procurement.
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: Vec<cac::ContractingParty>,
/// The party who originated Order.
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Vec<cac::CustomerParty>,
/// A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
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
    #[serde(rename = "ProcurementProject")]
    pub procurement_project: cac::ProcurementProject,
/// One of the procurement project lots into which this contract can be split.
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Vec<cac::ProcurementProjectLot>,
}
