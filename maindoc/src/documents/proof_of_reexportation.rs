#[derive(Debug, Deserialize, Serialize)]
/// A document providing a status or a proof that goods have been re-exported
///
/// UBL Dictionary Entry Name: `Proof Of Reexportation. Details`
///
/// Generated from XSD type `ProofOfReexportationType`.
pub struct ProofOfReexportation {
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
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies the current version of this request for proof
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// The Party who is competent of Customs in the exporting country.
    #[serde(default, rename = "ExportingCustomsParty")]
    pub exporting_customs_party: Option<cac::Party>,
/// The Party who provides the guarantee for the Goods while being temporarily imported. This Party is
/// normally a chamber of commerce.
    #[serde(default, rename = "ImportingGuarantorParty")]
    pub importing_guarantor_party: Option<cac::Party>,
/// The Party who provides the guarantee for the Goods while being temporarily exported. This Party is
/// normally a chamber of commerce.
    #[serde(default, rename = "ExportingGuarantorParty")]
    pub exporting_guarantor_party: Option<cac::Party>,
/// One or more goods item passport or ATA Carnet counterfoils associated with this proof of
/// re-exportation
    #[serde(default, rename = "GoodsItemPassportCounterfoil")]
    pub goods_item_passport_counterfoil: Vec<cac::GoodsItemPassportCounterfoil>,
/// One or more references to evidence supporting that goods have been re-exported
    #[serde(default, rename = "ReexportationEvidence")]
    pub reexportation_evidence: Vec<cac::Evidence>,
/// Attachment of the goods item passport or ATA Carnet related to this proof of re-exportation
    #[serde(default, rename = "GoodsItemPassportAttachment")]
    pub goods_item_passport_attachment: Option<cac::Attachment>,
/// One or more references to additional documents related to this proof of re-exportation
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
