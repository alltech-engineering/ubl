#[derive(Debug, Deserialize, Serialize)]
pub struct Code {
    #[serde(default, rename = "@listID")]
    pub list_id: Option<String>,
    #[serde(default, rename = "@listAgencyID")]
    pub list_agency_id: Option<String>,
    #[serde(default, rename = "@listAgencyName")]
    pub list_agency_name: Option<String>,
    #[serde(default, rename = "@listName")]
    pub list_name: Option<String>,
    #[serde(default, rename = "@listVersionID")]
    pub list_version_id: Option<String>,
    #[serde(default, rename = "@name")]
    pub name: Option<String>,
    #[serde(default, rename = "@languageID")]
    pub language_id: Option<String>,
    #[serde(default, rename = "@listURI")]
    pub list_uri: Option<String>,
    #[serde(default, rename = "@listSchemeURI")]
    pub list_scheme_uri: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
