#[derive(Debug, Deserialize, Serialize)]
pub struct SignedProperties {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "SignedSignatureProperties")]
    pub signed_signature_properties: Option<SignedSignatureProperties>,
    #[serde(default, rename = "SignedDataObjectProperties")]
    pub signed_data_object_properties: Option<SignedDataObjectProperties>,
}
