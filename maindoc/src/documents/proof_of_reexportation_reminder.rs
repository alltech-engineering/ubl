#[derive(Debug, Deserialize, Serialize)]
pub struct ProofOfReexportationReminder {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(rename = "ProcedureCode")]
    pub procedure_code: cct::CodeType,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "GoodsItemPassportID")]
    pub goods_item_passport_id: Option<cct::IdentifierType>,
    #[serde(rename = "ProofOfReexportationRequestDocumentReference")]
    pub proof_of_reexportation_request_document_reference: cac::DocumentReference,
    #[serde(rename = "ImportingGuarantorParty")]
    pub importing_guarantor_party: cac::Party,
    #[serde(rename = "ExportingGuarantorParty")]
    pub exporting_guarantor_party: cac::Party,
    #[serde(default, rename = "ImportingCustomsParty")]
    pub importing_customs_party: Option<cac::Party>,
    #[serde(default, rename = "IssuerEndorsement")]
    pub issuer_endorsement: Option<cac::Endorsement>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<cac::PaymentTerms>,
    #[serde(default, rename = "GoodsItemPassportCounterfoil")]
    pub goods_item_passport_counterfoil: Vec<cac::GoodsItemPassportCounterfoil>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
