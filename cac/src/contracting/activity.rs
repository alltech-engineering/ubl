#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingActivity {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ActivityTypeCode")]
    pub activity_type_code: Option<cct::Code>,
    #[serde(default, rename = "ActivityType")]
    pub activity_type: Vec<cct::Text>,
}
