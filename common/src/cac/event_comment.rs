#[derive(Debug, Deserialize, Serialize)]
pub struct EventComment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Comment")]
    pub comment: super::cct::TextType,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<super::udt::DateTimeType>,
}
