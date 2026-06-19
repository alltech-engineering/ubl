#[derive(Debug, Deserialize, Serialize)]
pub struct QualifyingProperties {
    #[serde(rename = "@Target")]
    pub target: String,
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "SignedProperties")]
    pub signed_properties: Option<SignedProperties>,
    #[serde(default, rename = "UnsignedProperties")]
    pub unsigned_properties: Option<UnsignedProperties>,
}
