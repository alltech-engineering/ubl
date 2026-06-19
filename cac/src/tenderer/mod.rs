use serde::{Deserialize, Serialize};


include!("party_qualification.rs");
include!("qualification_request.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct TendererRequirement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
    #[serde(default, rename = "TendererRequirementTypeCode")]
    pub tenderer_requirement_type_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "LegalReference")]
    pub legal_reference: Option<cct::Text>,
    #[serde(default, rename = "SuggestedEvidence")]
    pub suggested_evidence: Vec<crate::Evidence>,
}
