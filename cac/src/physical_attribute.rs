#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a physical attribute.
///
/// UBL Dictionary Entry Name: `Physical Attribute. Details`
///
/// Generated from XSD type `PhysicalAttributeType`.
pub struct PhysicalAttribute {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this physical attribute.
    #[serde(rename = "AttributeID")]
    pub attribute_id: cct::Identifier,
/// A code signifying the position of this physical attribute.
    #[serde(default, rename = "PositionCode")]
    pub position_code: Option<cct::Code>,
/// A description of the physical attribute, expressed as a code.
    #[serde(default, rename = "DescriptionCode")]
    pub description_code: Option<cct::Code>,
/// A description of the physical attribute, expressed as text.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
