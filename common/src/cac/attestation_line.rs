#[derive(Debug, Deserialize, Serialize)]
pub struct AttestationLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "CriterionItem")]
    pub criterion_item: Vec<CriterionItem>,
    #[serde(default, rename = "SubAttestationLine")]
    pub sub_attestation_line: Vec<AttestationLine>,
}
