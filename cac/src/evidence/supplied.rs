#[derive(Debug, Deserialize, Serialize)]
/// (Deprecated) A reference to evidence.
///
/// UBL Dictionary Entry Name: `Evidence Supplied. Details`
///
/// Generated from XSD type `EvidenceSuppliedType`.
pub struct EvidenceSupplied {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The identifier of the referenced evidence.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
}
