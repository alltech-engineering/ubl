#[derive(Debug, Deserialize, Serialize)]
pub struct WorkPhaseReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "WorkPhaseCode")]
    pub work_phase_code: Option<cct::Code>,
    #[serde(default, rename = "WorkPhase")]
    pub work_phase: Vec<cct::Text>,
    #[serde(default, rename = "ProgressPercent")]
    pub progress_percent: Option<cct::Numeric>,
    #[serde(default, rename = "StartDate")]
    pub start_date: Option<udt::DateTime>,
    #[serde(default, rename = "EndDate")]
    pub end_date: Option<udt::DateTime>,
    #[serde(default, rename = "WorkOrderDocumentReference")]
    pub work_order_document_reference: Vec<crate::DocumentReference>,
}
