#[derive(Debug, Deserialize, Serialize)]
pub enum SignaturePolicyIdentifier {
    #[serde(rename = "SignaturePolicyId")]
    SignaturePolicyId(SignaturePolicyIdType),
    #[serde(rename = "SignaturePolicyImplied")]
    SignaturePolicyImplied(String),
}
