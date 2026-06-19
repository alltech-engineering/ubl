#[derive(Debug, Deserialize, Serialize)]
/// A class that refers to a phase of work. Used for instance to specify what part of the contract the
/// billing is referring to.
///
/// UBL Dictionary Entry Name: `Work Phase Reference. Details`
///
/// Generated from XSD type `WorkPhaseReferenceType`.
pub struct WorkPhaseReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this phase of work.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code signifying this phase of work.
    #[serde(default, rename = "WorkPhaseCode")]
    pub work_phase_code: Option<cct::Code>,
/// Text describing this phase of work.
    #[serde(default, rename = "WorkPhase")]
    pub work_phase: Vec<cct::Text>,
/// The progress percentage of the work phase.
    #[serde(default, rename = "ProgressPercent")]
    pub progress_percent: Option<cct::Numeric>,
/// The date on which this phase of work begins.
    #[serde(default, rename = "StartDate")]
    pub start_date: Option<udt::DateTime>,
/// The date on which this phase of work ends.
    #[serde(default, rename = "EndDate")]
    pub end_date: Option<udt::DateTime>,
/// A reference to a document regarding the work order for the project in which this phase of work takes
/// place.
    #[serde(default, rename = "WorkOrderDocumentReference")]
    pub work_order_document_reference: Vec<crate::DocumentReference>,
}
