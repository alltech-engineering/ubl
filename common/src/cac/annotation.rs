#[derive(Debug, Deserialize, Serialize)]
pub struct Annotation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "SubjectCode")]
    pub subject_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Subject")]
    pub subject: Vec<super::cct::TextType>,
    #[serde(default, rename = "AnnotationContent")]
    pub annotation_content: Vec<super::cct::TextType>,
}
