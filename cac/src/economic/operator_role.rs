#[derive(Debug, Deserialize, Serialize)]
pub struct EconomicOperatorRole {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "RoleCode")]
    pub role_code: Option<cct::Code>,
    #[serde(default, rename = "RoleDescription")]
    pub role_description: Vec<cct::Text>,
}
