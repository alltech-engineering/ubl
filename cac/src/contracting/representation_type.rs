#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingRepresentationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "RepresentationTypeCode")]
    pub representation_type_code: Option<cct::Code>,
    #[serde(default, rename = "RepresentationType")]
    pub representation_type: Vec<cct::Text>,
}
