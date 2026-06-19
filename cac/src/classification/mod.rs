use serde::{Deserialize, Serialize};

include!("scheme.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to define a category within a classification scheme.
///
/// UBL Dictionary Entry Name: `Classification Category. Details`
///
/// Generated from XSD type `ClassificationCategoryType`.
pub struct ClassificationCategory {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The name of this category within the classification scheme.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// The value of a code used to identify this category within the classification scheme.
    #[serde(default, rename = "CodeValue")]
    pub code_value: Option<cct::Text>,
/// Text describing this category.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A recursive description of a subcategory of this category.
    #[serde(default, rename = "CategorizesClassificationCategory")]
    pub categorizes_classification_category: Vec<ClassificationCategory>,
}
