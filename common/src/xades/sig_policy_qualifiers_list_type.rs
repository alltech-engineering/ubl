#[derive(Debug, Deserialize, Serialize)]
pub struct SigPolicyQualifiersListType {
    #[serde(default, rename = "SigPolicyQualifier")]
    pub sig_policy_qualifier: Vec<Any>,
}
