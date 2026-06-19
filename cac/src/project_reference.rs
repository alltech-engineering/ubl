#[derive(Debug, Deserialize, Serialize)]
/// A class to define a reference to a procurement project.
///
/// UBL Dictionary Entry Name: `Project Reference. Details`
///
/// Generated from XSD type `ProjectReferenceType`.
pub struct ProjectReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the referenced project.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for the referenced project.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date on which the referenced project was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// A specific phase of work in the referenced project.
    #[serde(default, rename = "WorkPhaseReference")]
    pub work_phase_reference: Vec<WorkPhaseReference>,
}
