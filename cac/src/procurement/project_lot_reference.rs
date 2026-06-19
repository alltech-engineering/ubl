#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementProjectLotReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
}
