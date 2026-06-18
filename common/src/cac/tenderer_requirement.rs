#[derive(Debug, Deserialize, Serialize)]
pub struct TendererRequirement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Name")]
    pub name: Vec<super::cct::TextType>,
    #[serde(default, rename = "TendererRequirementTypeCode")]
    pub tenderer_requirement_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "LegalReference")]
    pub legal_reference: Option<super::cct::TextType>,
    #[serde(default, rename = "SuggestedEvidence")]
    pub suggested_evidence: Vec<Evidence>,
}
