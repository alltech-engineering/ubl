#[derive(Debug, Deserialize, Serialize)]
pub struct X509Data {
    #[serde(rename = "$value")]
    pub content: Vec<X509DataTypeContent>,
}
