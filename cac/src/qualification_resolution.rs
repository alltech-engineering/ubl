#[derive(Debug, Deserialize, Serialize)]
pub struct QualificationResolution {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "AdmissionCode")]
    pub admission_code: cct::Code,
    #[serde(default, rename = "ExclusionReason")]
    pub exclusion_reason: Vec<cct::Text>,
    #[serde(default, rename = "Resolution")]
    pub resolution: Vec<cct::Text>,
    #[serde(rename = "ResolutionDate")]
    pub resolution_date: udt::DateTime,
    #[serde(default, rename = "ResolutionTime")]
    pub resolution_time: Option<udt::DateTime>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Option<ProcurementProjectLot>,
}
