#[derive(Debug, Deserialize, Serialize)]
/// A template for a required document in a tendering process.
///
/// UBL Dictionary Entry Name: `Tender Requirement. Details`
///
/// Generated from XSD type `TenderRequirementType`.
pub struct TenderRequirement {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A name of this tender requirement.
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
/// Text describing this tender requirement.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A reference to the template for a required document.
    #[serde(default, rename = "TemplateDocumentReference")]
    pub template_document_reference: Option<crate::DocumentReference>,
}
