#[derive(Debug, Deserialize, Serialize)]
/// A class to reference to a lot identifier.
///
/// UBL Dictionary Entry Name: `Procurement Project Lot Reference. Details`
///
/// Generated from XSD type `ProcurementProjectLotReferenceType`.
pub struct ProcurementProjectLotReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this procurement project lot.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
}
