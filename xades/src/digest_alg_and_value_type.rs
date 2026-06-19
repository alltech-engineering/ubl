#[derive(Debug, Deserialize, Serialize)]
pub struct DigestAlgAndValueType {
    #[serde(rename = "DigestMethod")]
    pub digest_method: ds::DigestMethod,
    #[serde(rename = "DigestValue")]
    pub digest_value: String,
}
