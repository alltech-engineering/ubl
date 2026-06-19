#[derive(Debug, Deserialize, Serialize)]
/// A self-declaration of an economic operator, stating that it does not fall under the Exclusion
/// Grounds and that it meets the Selection Criteria for a specific procurement.
///
/// UBL Dictionary Entry Name: `Qualification Application Response. Details`
///
/// Generated from XSD type `QualificationApplicationResponseType`.
pub struct QualificationApplicationResponse {
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
/// Short title of a contract associated with this Tender.
    #[serde(default, rename = "ContractName")]
    pub contract_name: Vec<cct::Text>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Economic Operator Group Name associated with this Qualification.
    #[serde(default, rename = "EconomicOperatorGroupName")]
    pub economic_operator_group_name: Option<cct::Text>,
/// Indicates the current version of the Qualification Application Response.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// Identifies the previous version of the Qualification Application Response which is superceded by
/// this version.
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<cct::Identifier>,
/// A code signifying the type of this tendering procedure.
    #[serde(default, rename = "ProcedureCode")]
    pub procedure_code: Option<cct::Code>,
/// A code specifying the type of the Qualification Application.
    #[serde(default, rename = "QualificationApplicationTypeCode")]
    pub qualification_application_type_code: Option<cct::Code>,
/// Free-form text to describe Weight Scoring Methodology.
    #[serde(default, rename = "WeightScoringMethodologyNote")]
    pub weight_scoring_methodology_note: Vec<cct::Text>,
/// A code specifying the Weighting type
    #[serde(default, rename = "WeightingTypeCode")]
    pub weighting_type_code: Option<cct::Code>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The contracting party.
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: Vec<cac::ContractingParty>,
/// The Economic Operator issuing the Qualification Application Response.
    #[serde(default, rename = "EconomicOperatorParty")]
    pub economic_operator_party: Vec<cac::EconomicOperatorParty>,
/// An overall definition of this procurement project.
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: Option<cac::ProcurementProject>,
/// One of the procurement project lots into which this contract can be split.
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Vec<cac::ProcurementProjectLot>,
/// The criterion as described in the Qualification Application Request.
    #[serde(default, rename = "TenderingCriterion")]
    pub tendering_criterion: Vec<cac::TenderingCriterion>,
/// Each criterion requirement response.
    #[serde(default, rename = "TenderingCriterionResponse")]
    pub tendering_criterion_response: Vec<cac::TenderingCriterionResponse>,
/// A reference to an additional document associated with this document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// The evidence supporting this criterion requirement response.
    #[serde(default, rename = "Evidence")]
    pub evidence: Vec<cac::Evidence>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
