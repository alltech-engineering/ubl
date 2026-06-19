use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureInformation {
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "ReferencedSignatureID")]
    pub referenced_signature_id: Option<cct::Identifier>,
    #[serde(default, rename = "Signature")]
    pub signature: Option<ds::Signature>,
}
