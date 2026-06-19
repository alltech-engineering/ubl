#[derive(Debug, Deserialize, Serialize)]
pub struct ItemPropertyRange {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "MinimumValue")]
    pub minimum_value: Option<cct::Text>,
    #[serde(default, rename = "MaximumValue")]
    pub maximum_value: Option<cct::Text>,
}
