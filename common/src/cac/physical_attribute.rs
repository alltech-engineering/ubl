#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalAttribute {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "AttributeID")]
    pub attribute_id: super::cct::IdentifierType,
    #[serde(default, rename = "PositionCode")]
    pub position_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "DescriptionCode")]
    pub description_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
}
