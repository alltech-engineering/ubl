#[derive(Debug, Deserialize, Serialize)]
pub struct IdentifierType {
    #[serde(default, rename = "@schemeID")]
    pub scheme_id: Option<String>,
    #[serde(default, rename = "@schemeName")]
    pub scheme_name: Option<String>,
    #[serde(default, rename = "@schemeAgencyID")]
    pub scheme_agency_id: Option<String>,
    #[serde(default, rename = "@schemeAgencyName")]
    pub scheme_agency_name: Option<String>,
    #[serde(default, rename = "@schemeVersionID")]
    pub scheme_version_id: Option<String>,
    #[serde(default, rename = "@schemeDataURI")]
    pub scheme_data_uri: Option<String>,
    #[serde(default, rename = "@schemeURI")]
    pub scheme_uri: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
