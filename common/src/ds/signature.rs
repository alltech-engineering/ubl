#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "SignedInfo")]
    pub signed_info: SignedInfo,
    #[serde(rename = "SignatureValue")]
    pub signature_value: SignatureValue,
    #[serde(default, rename = "KeyInfo")]
    pub key_info: Option<KeyInfo>,
    #[serde(default, rename = "Object")]
    pub object: Vec<Object>,
}
