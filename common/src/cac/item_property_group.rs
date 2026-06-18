#[derive(Debug, Deserialize, Serialize)]
pub struct ItemPropertyGroup {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "ImportanceCode")]
    pub importance_code: Option<super::cct::CodeType>,
}
