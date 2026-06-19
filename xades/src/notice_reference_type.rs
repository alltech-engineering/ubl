#[derive(Debug, Deserialize, Serialize)]
pub struct NoticeReferenceType {
    #[serde(rename = "Organization")]
    pub organization: String,
    #[serde(rename = "NoticeNumbers")]
    pub notice_numbers: IntegerListType,
}
