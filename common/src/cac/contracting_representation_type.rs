#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingRepresentationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "RepresentationTypeCode")]
    pub representation_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "RepresentationType")]
    pub representation_type: Vec<super::cct::TextType>,
}
