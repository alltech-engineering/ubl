#[derive(Debug, Deserialize, Serialize)]
pub struct Regulation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
    #[serde(default, rename = "LegalReference")]
    pub legal_reference: Option<cct::Text>,
    #[serde(default, rename = "OntologyURI")]
    pub ontology_uri: Option<cct::Identifier>,
}
