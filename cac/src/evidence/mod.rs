use serde::{Deserialize, Serialize};


include!("supplied.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Evidence {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "EvidenceTypeCode")]
    pub evidence_type_code: Option<cct::Code>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "CandidateStatement")]
    pub candidate_statement: Vec<cct::Text>,
    #[serde(default, rename = "ConfidentialityLevelCode")]
    pub confidentiality_level_code: Option<cct::Code>,
    #[serde(default, rename = "EvidenceIssuingParty")]
    pub evidence_issuing_party: Option<crate::Party>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "Language")]
    pub language: Option<crate::Language>,
}
