use serde::{Deserialize, Serialize};


include!("extension.rs");
include!("execution_requirement.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Contract {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "NominationDate")]
    pub nomination_date: Option<udt::DateTime>,
    #[serde(default, rename = "NominationTime")]
    pub nomination_time: Option<udt::DateTime>,
    #[serde(default, rename = "ContractTypeCode")]
    pub contract_type_code: Option<cct::Code>,
    #[serde(default, rename = "ContractType")]
    pub contract_type: Option<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
    #[serde(default, rename = "ModificationReasonCode")]
    pub modification_reason_code: Option<cct::Code>,
    #[serde(default, rename = "ModificationReasonDescription")]
    pub modification_reason_description: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<crate::Period>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "NominationPeriod")]
    pub nomination_period: Option<crate::Period>,
    #[serde(default, rename = "ContractualDelivery")]
    pub contractual_delivery: Option<crate::Delivery>,
}
