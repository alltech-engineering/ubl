#[derive(Debug, Deserialize, Serialize)]
/// A class to define a structured annotation providing contextual or explanatory information related to
/// a document or other business object
///
/// UBL Dictionary Entry Name: `Annotation. Details`
///
/// Generated from XSD type `AnnotationType`.
pub struct Annotation {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code identifying the subject of the Annotation.
    #[serde(default, rename = "SubjectCode")]
    pub subject_code: Option<cct::Code>,
/// A textual description identifying the subject of the Annotation.
    #[serde(default, rename = "Subject")]
    pub subject: Vec<cct::Text>,
/// The textual content of the annotation providing information or context.
    #[serde(default, rename = "AnnotationContent")]
    pub annotation_content: Vec<cct::Text>,
}
