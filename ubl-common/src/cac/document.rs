// UBL Document Reference aggregate.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentReference {
    pub id: ID,
    pub copy_indicator: Option<CopyIndicator>,
    pub uuid: Option<UUID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub document_type_code: Option<DocumentTypeCode>,
    pub document_type: Option<DocumentType>,
    pub document_description: Vec<Description>,
    pub attachment: Option<Attachment>,
    pub validity_period: Option<Period>,
    pub issuer_party: Option<Party>,
    pub result_of_verification: Option<ResultOfVerification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attachment {
    pub embedded_document_binary_object: Option<EmbeddedDocumentBinaryObject>,
    pub external_reference: Option<ExternalReference>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalReference {
    pub uri: Option<Text>,
    pub description: Vec<Description>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultOfVerification {
    pub validator_id: Option<ValidatorID>,
    pub validation_result_code: Option<ValidationResultCode>,
    pub validation_date: Option<ValidationDate>,
    pub validation_time: Option<ValidationTime>,
}

use crate::cac::party::Party;
use crate::cac::period::Period;
