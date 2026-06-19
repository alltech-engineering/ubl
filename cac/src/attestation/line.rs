#[derive(Debug, Deserialize, Serialize)]
pub struct AttestationLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "CriterionItem")]
    pub criterion_item: Vec<crate::CriterionItem>,
    #[serde(default, rename = "SubAttestationLine")]
    pub sub_attestation_line: Vec<AttestationLine>,
}
