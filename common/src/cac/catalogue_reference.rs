#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "RevisionDate")]
    pub revision_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "RevisionTime")]
    pub revision_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<super::cct::IdentifierType>,
}
