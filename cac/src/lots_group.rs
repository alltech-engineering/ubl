#[derive(Debug, Deserialize, Serialize)]
/// A class for defining set of lots.
///
/// UBL Dictionary Entry Name: `Lots Group. Details`
///
/// Generated from XSD type `LotsGroupType`.
pub struct LotsGroup {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the lotsgroup.
    #[serde(rename = "LotsGroupID")]
    pub lots_group_id: cct::Identifier,
/// A Procurement project lot that is included in this LotsGroup.
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: Vec<ProcurementProjectLotReference>,
}
