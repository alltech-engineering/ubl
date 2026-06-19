#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a declaration by an economic operator of certain characteristics or capabilities
/// in fulfilment of requirements specified in a call for tenders.
///
/// UBL Dictionary Entry Name: `Declaration. Details`
///
/// Generated from XSD type `DeclarationType`.
pub struct Declaration {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this Declaration.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The name of this declaration.
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
/// A code signifying the type of this declaration.
    #[serde(default, rename = "DeclarationTypeCode")]
    pub declaration_type_code: Option<cct::Code>,
/// Text describing this declaration.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// (Deprecated) The evidence supporting this declaration.
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: Vec<EvidenceSupplied>,
/// The Evidence supporting this declaration.
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: Vec<Evidence>,
}
