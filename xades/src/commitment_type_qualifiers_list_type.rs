#[derive(Debug, Deserialize, Serialize)]
pub struct CommitmentTypeQualifiersListType {
    #[serde(default, rename = "CommitmentTypeQualifier")]
    pub commitment_type_qualifier: Vec<Any>,
}
