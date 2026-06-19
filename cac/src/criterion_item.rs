#[derive(Debug, Deserialize, Serialize)]
/// A class describing a criteria
///
/// UBL Dictionary Entry Name: `Criterion Item. Details`
///
/// Generated from XSD type `CriterionItemType`.
pub struct CriterionItem {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this criteria
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code describing the type of criteria
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::Code>,
/// The criteria for this item, expressed as a text
    #[serde(default, rename = "CriterionDescription")]
    pub criterion_description: Vec<cct::Text>,
/// The item associated with this criteria
    #[serde(rename = "DeclaredPropertyItem")]
    pub declared_property_item: Item,
}
