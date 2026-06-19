#[derive(Debug, Deserialize, Serialize)]
pub struct LotsGroup {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "LotsGroupID")]
    pub lots_group_id: cct::Identifier,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: Vec<ProcurementProjectLotReference>,
}
