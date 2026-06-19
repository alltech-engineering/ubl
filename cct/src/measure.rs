#[derive(Debug, Deserialize, Serialize)]
pub struct Measure {
    #[serde(default, rename = "@unitCode")]
    pub unit_code: Option<String>,
    #[serde(default, rename = "@unitCodeListVersionID")]
    pub unit_code_list_version_id: Option<String>,
    #[serde(rename = "$text")]
    pub content: f64,
}
