#[derive(Debug, Deserialize, Serialize)]
/// A reminder that a requested Proof of Reexportation is pending.
///
/// UBL Dictionary Entry Name: `Proof Of Reexportation Reminder. Details`
///
/// Generated from XSD type `ProofOfReexportationReminderType`.
pub struct ProofOfReexportationReminder {
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
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// The procedure under which this reminder was sent, expressed as a code
    #[serde(rename = "ProcedureCode")]
    pub procedure_code: cct::Code,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies a version of a Proof of Reexportation Reminder in order to distinguish updates.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// An identifier for the associated Goods Item Passport, used when all counterfoils refer to the same.
    #[serde(default, rename = "GoodsItemPassportID")]
    pub goods_item_passport_id: Option<cct::Identifier>,
/// The Document Reference related to this Proof of Reexportation Request
    #[serde(rename = "ProofOfReexportationRequestDocumentReference")]
    pub proof_of_reexportation_request_document_reference: cac::DocumentReference,
/// The Party who on behalf of their Customs Authority issues this Document. This Party is normally a
/// chamber of commerce.
    #[serde(rename = "ImportingGuarantorParty")]
    pub importing_guarantor_party: cac::Party,
/// The Party who is fiscally responsible for the Goods Item Passport counterfoils which the Customs
/// Party is requesting. This Party is normally a chamber of commerce.
    #[serde(rename = "ExportingGuarantorParty")]
    pub exporting_guarantor_party: cac::Party,
/// The Party who originally requests the Proof of Reexportation.
    #[serde(default, rename = "ImportingCustomsParty")]
    pub importing_customs_party: Option<cac::Party>,
/// An Issuers endorsment of this Request for Proof of Reexportation.
    #[serde(default, rename = "IssuerEndorsement")]
    pub issuer_endorsement: Option<cac::Endorsement>,
/// A set of payment terms associated with this Request for Proof of Reexportation, used for generating
/// a subsequent invoice in case no proof of re-exportation can be provided.
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<cac::PaymentTerms>,
/// The related Goods Item Passport Counterfoils of an associated Goods Item Passport.
    #[serde(default, rename = "GoodsItemPassportCounterfoil")]
    pub goods_item_passport_counterfoil: Vec<cac::GoodsItemPassportCounterfoil>,
/// One or more references to additional documents related to this Request for Proof of Reexportation
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
