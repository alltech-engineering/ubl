#[derive(Debug, Deserialize, Serialize)]
pub struct QuantityType {
    #[serde(default, rename = "@unitCode")]
    pub unit_code: Option<String>,
    #[serde(default, rename = "@unitCodeListID")]
    pub unit_code_list_id: Option<String>,
    #[serde(default, rename = "@unitCodeListAgencyID")]
    pub unit_code_list_agency_id: Option<String>,
    #[serde(default, rename = "@unitCodeListAgencyName")]
    pub unit_code_list_agency_name: Option<String>,
    #[serde(rename = "$text")]
    pub content: f64,
}
