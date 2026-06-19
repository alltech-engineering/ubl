#[derive(Debug, Deserialize, Serialize)]
pub struct CriterionItem {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::Code>,
    #[serde(default, rename = "CriterionDescription")]
    pub criterion_description: Vec<cct::Text>,
    #[serde(rename = "DeclaredPropertyItem")]
    pub declared_property_item: Item,
}
