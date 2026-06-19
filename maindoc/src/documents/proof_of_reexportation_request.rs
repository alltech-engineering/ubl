#[derive(Debug, Deserialize, Serialize)]
/// A document requesting the status or proof that goods have been re-exported
///
/// UBL Dictionary Entry Name: `Proof Of Reexportation Request. Details`
///
/// Generated from XSD type `ProofOfReexportationRequestType`.
pub struct ProofOfReexportationRequest {
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
/// The identifier of the goods item passport or ATA Carnet of the goods
    #[serde(rename = "GoodsItemPassportID")]
    pub goods_item_passport_id: cct::Identifier,
/// A reference to a counterfoil of the goods item passport or ATA Carnet
    #[serde(default, rename = "GoodsItemPassportCounterfoilID")]
    pub goods_item_passport_counterfoil_id: Option<cct::Identifier>,
/// The Party who on behalf of their Customs Authority issues the Proof of Reexportation. This Party is
/// normally a chamber of commerce.
    #[serde(rename = "ImportingGuarantorParty")]
    pub importing_guarantor_party: cac::Party,
/// The Party who is fiscally responsible for the Goods Item Passport counterfoils which the Customs
/// Party is requesting. This Party is normally a chamber of commerce.
    #[serde(rename = "ExportingGuarantorParty")]
    pub exporting_guarantor_party: cac::Party,
/// The Party who originally requests the Proof of Reexportation.
    #[serde(default, rename = "ImportingCustomsParty")]
    pub importing_customs_party: Option<cac::Party>,
/// One or more references to additional documents related to this request
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
