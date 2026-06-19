#[derive(Debug, Deserialize, Serialize)]
pub struct ClassificationScheme {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "LastRevisionDate")]
    pub last_revision_date: Option<udt::DateTime>,
    #[serde(default, rename = "LastRevisionTime")]
    pub last_revision_time: Option<udt::DateTime>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "AgencyID")]
    pub agency_id: Option<cct::Identifier>,
    #[serde(default, rename = "AgencyName")]
    pub agency_name: Option<cct::Text>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
    #[serde(default, rename = "URI")]
    pub uri: Option<cct::Identifier>,
    #[serde(default, rename = "SchemeURI")]
    pub scheme_uri: Option<cct::Identifier>,
    #[serde(default, rename = "LanguageID")]
    pub language_id: Option<cct::Identifier>,
    #[serde(default, rename = "ClassificationCategory")]
    pub classification_category: Vec<ClassificationCategory>,
}
