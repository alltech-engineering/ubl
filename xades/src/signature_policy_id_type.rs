#[derive(Debug, Deserialize, Serialize)]
pub struct SignaturePolicyIdType {
    #[serde(rename = "SigPolicyId")]
    pub sig_policy_id: ObjectIdentifier,
    #[serde(default, rename = "Transforms")]
    pub transforms: Option<ds::Transforms>,
    #[serde(rename = "SigPolicyHash")]
    pub sig_policy_hash: DigestAlgAndValueType,
    #[serde(default, rename = "SigPolicyQualifiers")]
    pub sig_policy_qualifiers: Option<SigPolicyQualifiersListType>,
}
