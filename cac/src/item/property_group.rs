#[derive(Debug, Deserialize, Serialize)]
pub struct ItemPropertyGroup {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "ImportanceCode")]
    pub importance_code: Option<cct::Code>,
}
