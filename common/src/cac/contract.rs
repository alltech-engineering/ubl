#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "NominationDate")]
    pub nomination_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "NominationTime")]
    pub nomination_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ContractTypeCode")]
    pub contract_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ContractType")]
    pub contract_type: Option<super::cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ModificationReasonCode")]
    pub modification_reason_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ModificationReasonDescription")]
    pub modification_reason_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "NominationPeriod")]
    pub nomination_period: Option<Period>,
    #[serde(default, rename = "ContractualDelivery")]
    pub contractual_delivery: Option<Delivery>,
}
