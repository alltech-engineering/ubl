#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingRepresentation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "RepresentationTypeCode")]
    pub representation_type_code: Option<cct::Code>,
    #[serde(default, rename = "RepresentationType")]
    pub representation_type: Vec<cct::Text>,
}
