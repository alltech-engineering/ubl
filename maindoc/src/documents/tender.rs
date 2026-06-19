#[derive(Debug, Deserialize, Serialize)]
/// A document whereby an economic operator (the tenderer) makes a formal offer (the tender) to a
/// contracting authority to execute an order for the supply or purchase of goods, or for the execution
/// of work, according to the terms of a proposed contract.
///
/// UBL Dictionary Entry Name: `Tender. Details`
///
/// Generated from XSD type `TenderType`.
pub struct Tender {
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
/// A code to specify the type of tender (economical or objective criteria versus technical or
/// subjective criteria)
    #[serde(default, rename = "TenderTypeCode")]
    pub tender_type_code: Option<cct::Code>,
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
/// The period for which the Tender is valid.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<cac::Period>,
/// A reference to the call for tender document of which this tender result of.
    #[serde(default, rename = "CallForTenderDocumentReference")]
    pub call_for_tender_document_reference: Option<cac::DocumentReference>,
/// A reference to another document associated with this document.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who submits this Tender.
    #[serde(default, rename = "TendererParty")]
    pub tenderer_party: Vec<cac::Party>,
/// A reference to the tenderer qualification document that has been used to qualify the tenderer.
    #[serde(default, rename = "TendererQualificationDocumentReference")]
    pub tenderer_qualification_document_reference:
        Option<cac::DocumentReference>,
/// The Subcontractor or other Tenderer who participates in the same Tender.
    #[serde(default, rename = "SubcontractorParty")]
    pub subcontractor_party: Vec<cac::Party>,
/// The contracting party.
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: Vec<cac::ContractingParty>,
/// The party originating the Tender.
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<cac::CustomerParty>,
/// A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<cac::Party>,
/// A project with which this Tender is associated. A single Tender can be used to bid for one project,
/// multiple projects, or the global project.
    #[serde(default, rename = "TenderedProject")]
    pub tendered_project: Vec<cac::TenderedProject>,
}
