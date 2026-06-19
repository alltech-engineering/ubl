#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementAdditional {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ProcurementTypeCode")]
    pub procurement_type_code: Option<cct::Code>,
    #[serde(default, rename = "ProcurementType")]
    pub procurement_type: Vec<cct::Text>,
}
