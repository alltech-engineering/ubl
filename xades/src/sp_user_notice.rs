#[derive(Debug, Deserialize, Serialize)]
pub struct SpUserNotice {
    #[serde(default, rename = "NoticeRef")]
    pub notice_ref: Option<NoticeReferenceType>,
    #[serde(default, rename = "ExplicitText")]
    pub explicit_text: Option<String>,
}
