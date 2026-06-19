#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingActivity {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ActivityTypeCode")]
    pub activity_type_code: Option<cct::Code>,
    #[serde(default, rename = "ActivityType")]
    pub activity_type: Vec<cct::Text>,
}
