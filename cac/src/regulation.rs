#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a regulation.
///
/// UBL Dictionary Entry Name: `Regulation. Details`
///
/// Generated from XSD type `RegulationType`.
pub struct Regulation {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A name for this regulation.
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
/// Text describing a legal reference.
    #[serde(default, rename = "LegalReference")]
    pub legal_reference: Option<cct::Text>,
/// The Uniform Resource Identifier (URI) of an ontology related to this regulation.
    #[serde(default, rename = "OntologyURI")]
    pub ontology_uri: Option<cct::Identifier>,
}
