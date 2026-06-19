#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalAttribute {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "AttributeID")]
    pub attribute_id: cct::Identifier,
    #[serde(default, rename = "PositionCode")]
    pub position_code: Option<cct::Code>,
    #[serde(default, rename = "DescriptionCode")]
    pub description_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
