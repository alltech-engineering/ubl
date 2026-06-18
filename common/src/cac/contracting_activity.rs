#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingActivity {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ActivityTypeCode")]
    pub activity_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ActivityType")]
    pub activity_type: Vec<super::cct::TextType>,
}
