#[derive(Debug, Deserialize, Serialize)]
/// A class describing an attestation line
///
/// UBL Dictionary Entry Name: `Attestation Line. Details`
///
/// Generated from XSD type `AttestationLineType`.
pub struct AttestationLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this attestation line
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code describing the type of attestation line or statement
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::Code>,
/// A textual description of this attestation line
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Criterion items associated with this attestation line
    #[serde(default, rename = "CriterionItem")]
    pub criterion_item: Vec<crate::CriterionItem>,
/// An attestation line subsidiary to this attestation line
    #[serde(default, rename = "SubAttestationLine")]
    pub sub_attestation_line: Vec<AttestationLine>,
}
