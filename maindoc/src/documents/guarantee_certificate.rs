#[derive(Debug, Deserialize, Serialize)]
/// A document to notify the deposit of a Guarantee, such as a bid bond.
///
/// UBL Dictionary Entry Name: `Guarantee Certificate. Details`
///
/// Generated from XSD type `GuaranteeCertificateType`.
pub struct GuaranteeCertificate {
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
/// A code signifying the type of the Guarantee.
    #[serde(default, rename = "GuaranteeTypeCode")]
    pub guarantee_type_code: Option<cct::Code>,
/// A textual description of the purpose of the Guarantee.
    #[serde(default, rename = "Purpose")]
    pub purpose: Vec<cct::Text>,
/// The liability amount (a monetary value) in the Guarantee.
    #[serde(rename = "LiabilityAmount")]
    pub liability_amount: cct::Amount,
/// The code stating the constitution means of the Guarantee.
    #[serde(default, rename = "ConstitutionCode")]
    pub constitution_code: Option<cct::Code>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The specified period in the tendering process for which this Guarantee is effective
    #[serde(default, rename = "ApplicablePeriod")]
    pub applicable_period: Option<cac::Period>,
/// A reference to an applicable regulation.
    #[serde(default, rename = "ApplicableRegulation")]
    pub applicable_regulation: Vec<cac::Regulation>,
/// A reference to a legal document.
    #[serde(default, rename = "GuaranteeDocumentReference")]
    pub guarantee_document_reference: Vec<cac::DocumentReference>,
/// Details of an immobilized security.
    #[serde(default, rename = "ImmobilizedSecurity")]
    pub immobilized_security: Vec<cac::ImmobilizedSecurity>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The guarantee creditor organisation who has the authority to charge bid bond guarantee credit.
    #[serde(rename = "GuarantorParty")]
    pub guarantor_party: cac::Party,
/// The Party who deposits the bid bond guarantee.
    #[serde(rename = "InterestedParty")]
    pub interested_party: cac::Party,
/// The recipient who benefits from the bid bond guarantee.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Option<cac::Party>,
}
