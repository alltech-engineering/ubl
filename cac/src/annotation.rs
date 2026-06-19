#[derive(Debug, Deserialize, Serialize)]
pub struct Annotation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "SubjectCode")]
    pub subject_code: Option<cct::Code>,
    #[serde(default, rename = "Subject")]
    pub subject: Vec<cct::Text>,
    #[serde(default, rename = "AnnotationContent")]
    pub annotation_content: Vec<cct::Text>,
}
