#[derive(Debug, Deserialize, Serialize)]
pub struct ClassificationScheme {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "LastRevisionDate")]
    pub last_revision_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "LastRevisionTime")]
    pub last_revision_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "AgencyID")]
    pub agency_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AgencyName")]
    pub agency_name: Option<super::cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "URI")]
    pub uri: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SchemeURI")]
    pub scheme_uri: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "LanguageID")]
    pub language_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ClassificationCategory")]
    pub classification_category: Vec<ClassificationCategory>,
}
