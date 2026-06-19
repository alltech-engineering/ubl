#[derive(Debug, Deserialize, Serialize)]
pub struct Regulation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "Name")]
    pub name: Vec<cct::Text>,
    #[serde(default, rename = "LegalReference")]
    pub legal_reference: Option<cct::Text>,
    #[serde(default, rename = "OntologyURI")]
    pub ontology_uri: Option<cct::Identifier>,
}
