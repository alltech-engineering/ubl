#[derive(Debug, Deserialize, Serialize)]
pub struct SignedInfo {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "CanonicalizationMethod")]
    pub canonicalization_method: CanonicalizationMethod,
    #[serde(rename = "SignatureMethod")]
    pub signature_method: SignatureMethod,
    #[serde(default, rename = "Reference")]
    pub reference: Vec<Reference>,
}
