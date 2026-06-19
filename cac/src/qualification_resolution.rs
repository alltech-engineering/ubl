#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the acceptance or rejection of an economic operator in a tendering process.
///
/// UBL Dictionary Entry Name: `Qualification Resolution. Details`
///
/// Generated from XSD type `QualificationResolutionType`.
pub struct QualificationResolution {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An indicator that the economic operator has been accepted into the tendering process (true) or
/// rejected from the tendering process (false).
    #[serde(rename = "AdmissionCode")]
    pub admission_code: cct::Code,
/// Text describing a reason for an exclusion from the tendering process.
    #[serde(default, rename = "ExclusionReason")]
    pub exclusion_reason: Vec<cct::Text>,
/// Text describing this qualification resolution.
    #[serde(default, rename = "Resolution")]
    pub resolution: Vec<cct::Text>,
/// The date on which this qualification resolution was formalized.
    #[serde(rename = "ResolutionDate")]
    pub resolution_date: udt::DateTime,
/// The time at which this qualification resolution was formalized.
    #[serde(default, rename = "ResolutionTime")]
    pub resolution_time: Option<udt::DateTime>,
/// The Procurement project lot to which this tenderer is accepted or rejected.
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: Option<ProcurementProjectLot>,
}
