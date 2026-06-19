#[derive(Debug, Deserialize, Serialize)]
pub struct CommitmentTypeIndication {
    #[serde(rename = "$value")]
    pub content: Vec<CommitmentTypeIndicationTypeContent>,
}
