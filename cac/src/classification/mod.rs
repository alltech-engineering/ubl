use serde::{Deserialize, Serialize};


include!("scheme.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct ClassificationCategory {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "CodeValue")]
    pub code_value: Option<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "CategorizesClassificationCategory")]
    pub categorizes_classification_category: Vec<ClassificationCategory>,
}
