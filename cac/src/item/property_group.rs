#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a property group or classification.
///
/// UBL Dictionary Entry Name: `Item Property Group. Details`
///
/// Generated from XSD type `ItemPropertyGroupType`.
pub struct ItemPropertyGroup {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this group of item properties.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// The name of this item property group.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// A code signifying the importance of this property group in using it to describe a required Item.
    #[serde(default, rename = "ImportanceCode")]
    pub importance_code: Option<cct::Code>,
}
