#[derive(Debug, Deserialize, Serialize)]
/// A request to unsubscribe from a tendering procedure. Economic Operators can subscribe to a tendering
/// procedure using the Expression Of Interest. Upon subscription, the Economic Operator keeps receiving
/// relevant documentation for the tendering process. The unsubscribe to procedure document allows the
/// Economic Operator to be removed from the list of interested parties.
///
/// UBL Dictionary Entry Name: `Unsubscribe From Procedure Request. Details`
///
/// Generated from XSD type `UnsubscribeFromProcedureRequestType`.
pub struct UnsubscribeFromProcedureRequest {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
/// The earliest version of the UBL 2 schema for this document type that defines all of the elements
/// that might be encountered in the current instance.
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
/// An identifier, assigned by the sender, for the process file (i.e., record) to which this document
/// belongs.
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: Option<cct::Identifier>,
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
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Economic Operator issuing this unsubscribe request.
    #[serde(rename = "EconomicOperatorParty")]
    pub economic_operator_party: cac::EconomicOperatorParty,
/// The Contracting Party.
    #[serde(rename = "ContractingParty")]
    pub contracting_party: cac::ContractingParty,
/// An overall definition of this procurement project.
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: Option<cac::ProcurementProject>,
/// One of the procurement project lots into which this contract can be split.
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: Vec<cac::ProcurementProjectLotReference>,
}
