use serde::{Deserialize, Serialize};
pub type SignatureInformation = SignatureInformationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureInformationType {
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ReferencedSignatureID")]
    pub referenced_signature_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::core::option::Option<super::ds::ubl_xmldsig_core_schema_25::SignatureType>,
}
