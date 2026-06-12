// UBL Signature aggregate.
// A digital or physical signature applied to a document.

use serde::{Deserialize, Serialize};
use crate::cbc::*;
use crate::cac::party::Party;

/// A signature applied to a UBL document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    pub id: ID,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_date: Option<ValidationDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_time: Option<ValidationTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validator_id: Option<ValidatorID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_method_code: Option<SignatureMethodCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signatory_party: Option<Party>,
    // TODO: CanonicalizationMethod, SignatureMethod, DigitalSignatureAttachment, OriginalDocumentReference
}
