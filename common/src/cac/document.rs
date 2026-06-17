// UBL Document aggregate — DocumentDistribution, and re-exports from document_reference
// and signature.
//
// DocumentReference, Attachment, ExternalReference, and ResultOfVerification
// are defined canonically in document_reference.rs and re-exported here for
// backward compatibility with existing imports.
// Signature is defined canonically in signature.rs.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

// Re-export the canonical definitions from document_reference
pub use super::document_reference::{
    Attachment, DocumentReference, ExternalReference, ResultOfVerification,
};
// Re-export Signature from its canonical module
pub use super::signature::{DigitalSignatureAttachment, Signature};

// ─── DocumentDistribution ─────────────────────────────────────────────
// UBL CAC DocumentDistributionType

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentDistribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_qualifier: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_copies_numeric: Option<MaximumCopiesNumeric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<Party>,
}

use crate::cac::party::Party;
