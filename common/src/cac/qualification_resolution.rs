#[derive(Debug, Deserialize, Serialize)]
pub struct QualificationResolution {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "AdmissionCode")]
    pub admission_code: super::cct::CodeType,
    #[serde(default, rename = "ExclusionReason")]
    pub exclusion_reason: Vec<super::cct::TextType>,
    #[serde(default, rename = "Resolution")]
    pub resolution: Vec<super::cct::TextType>,
    #[serde(rename = "ResolutionDate")]
    pub resolution_date: super::udt::DateTimeType,
    #[serde(default, rename = "ResolutionTime")]
    pub resolution_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Option<ProcurementProjectLot>,
}
