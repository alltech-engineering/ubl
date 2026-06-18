#[derive(Debug, Deserialize, Serialize)]
pub struct LotsGroup {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "LotsGroupID")]
    pub lots_group_id: super::cct::IdentifierType,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: Vec<ProcurementProjectLotReference>,
}
