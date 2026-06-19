#[derive(Debug, Deserialize, Serialize)]
pub enum CommitmentTypeIndicationTypeContent {
    #[serde(rename = "CommitmentTypeId")]
    CommitmentTypeId(ObjectIdentifier),
    #[serde(rename = "ObjectReference")]
    ObjectReference(String),
    #[serde(rename = "AllSignedDataObjects")]
    AllSignedDataObjects(String),
    #[serde(rename = "CommitmentTypeQualifiers")]
    CommitmentTypeQualifiers(CommitmentTypeQualifiersListType),
}
