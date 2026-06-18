#[derive(Debug, Deserialize, Serialize)]
pub struct Evidence {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "EvidenceTypeCode")]
    pub evidence_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "CandidateStatement")]
    pub candidate_statement: Vec<super::cct::TextType>,
    #[serde(default, rename = "ConfidentialityLevelCode")]
    pub confidentiality_level_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "EvidenceIssuingParty")]
    pub evidence_issuing_party: Option<Party>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "Language")]
    pub language: Option<Language>,
}
