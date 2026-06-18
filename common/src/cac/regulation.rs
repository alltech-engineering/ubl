#[derive(Debug, Deserialize, Serialize)]
pub struct Regulation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Name")]
    pub name: Vec<super::cct::TextType>,
    #[serde(default, rename = "LegalReference")]
    pub legal_reference: Option<super::cct::TextType>,
    #[serde(default, rename = "OntologyURI")]
    pub ontology_uri: Option<super::cct::IdentifierType>,
}
