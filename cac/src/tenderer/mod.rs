use serde::{Deserialize, Serialize};


include!("party_qualification.rs");
include!("qualification_request.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an action or statement required of an economic operator participating in a
/// tendering process.
///
/// UBL Dictionary Entry Name: `Tenderer Requirement. Details`
///
/// Generated from XSD type `TendererRequirementType`.
pub struct TendererRequirement {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A name of this tenderer requirement.
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
/// A code signifying this requirement.
    #[serde(default, rename = "TendererRequirementTypeCode")]
    pub tenderer_requirement_type_code: Option<cct::Code>,
/// Text describing this requirement.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The legal reference of the exclusion criterion.
    #[serde(default, rename = "LegalReference")]
    pub legal_reference: Option<cct::Text>,
/// An item of evidence that ought to be submitted to satisfy this requirement.
    #[serde(default, rename = "SuggestedEvidence")]
    pub suggested_evidence: Vec<crate::Evidence>,
}
