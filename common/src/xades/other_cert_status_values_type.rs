#[derive(Debug, Deserialize, Serialize)]
pub struct OtherCertStatusValuesType {
    #[serde(default, rename = "OtherValue")]
    pub other_value: Vec<Any>,
}
