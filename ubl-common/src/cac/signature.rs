// UBL Signature aggregate.
// A digital or physical signature applied to a document.
//
// UBL 2.5 Section 6 — UBL Digital Signatures.

use crate::cac::document::{DocumentReference, ExternalReference};
use crate::cac::party::Party;
use crate::cbc::*;
use serde::{Deserialize, Serialize};

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
    pub canonicalization_method: Option<CanonicalizationMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_method: Option<SignatureMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signatory_party: Option<Party>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digital_signature_attachment: Option<DigitalSignatureAttachment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_document_reference: Option<Box<DocumentReference>>,
}

/// External reference to a detached digital signature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalSignatureAttachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<ExternalReference>,
}
