#[derive(Debug, Deserialize, Serialize)]
pub struct WorkPhaseReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "WorkPhaseCode")]
    pub work_phase_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "WorkPhase")]
    pub work_phase: Vec<super::cct::TextType>,
    #[serde(default, rename = "ProgressPercent")]
    pub progress_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "StartDate")]
    pub start_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EndDate")]
    pub end_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "WorkOrderDocumentReference")]
    pub work_order_document_reference: Vec<DocumentReference>,
}
