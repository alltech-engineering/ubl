#[derive(Debug, Deserialize, Serialize)]
pub struct Declaration {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
    #[serde(default, rename = "DeclarationTypeCode")]
    pub declaration_type_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: Vec<EvidenceSupplied>,
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: Vec<Evidence>,
}
