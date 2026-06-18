#[derive(Debug, Deserialize, Serialize)]
pub struct WorkQuantityTotal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Quantity")]
    pub quantity: super::cct::QuantityType,
    #[serde(default, rename = "WorkTypeCode")]
    pub work_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "WorkTypeDescription")]
    pub work_type_description: Vec<super::cct::TextType>,
}
