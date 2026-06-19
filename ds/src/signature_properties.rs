#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureProperties {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "SignatureProperty")]
    pub signature_property: Vec<SignatureProperty>,
}
