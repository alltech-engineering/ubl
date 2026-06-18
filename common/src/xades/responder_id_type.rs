#[derive(Debug, Deserialize, Serialize)]
pub enum ResponderIdType {
    #[serde(rename = "ByName")]
    ByName(String),
    #[serde(rename = "ByKey")]
    ByKey(String),
}
