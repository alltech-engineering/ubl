#[derive(Debug, Deserialize, Serialize)]
pub struct ClassificationCategory {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "CodeValue")]
    pub code_value: Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "CategorizesClassificationCategory")]
    pub categorizes_classification_category: Vec<ClassificationCategory>,
}
