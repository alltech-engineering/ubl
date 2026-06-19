#[derive(Debug, Deserialize, Serialize)]
/// A class to define comments about a retail event.
///
/// UBL Dictionary Entry Name: `Event Comment. Details`
///
/// Generated from XSD type `EventCommentType`.
pub struct EventComment {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// Text commenting on the event.
    #[serde(rename = "Comment")]
    pub comment: cct::Text,
/// The date on which this comment was made.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time at which this comment was made.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
}
