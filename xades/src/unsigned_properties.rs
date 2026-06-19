#[derive(Debug, Deserialize, Serialize)]
pub struct UnsignedProperties {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "UnsignedSignatureProperties")]
    pub unsigned_signature_properties: Option<UnsignedSignatureProperties>,
    #[serde(default, rename = "UnsignedDataObjectProperties")]
    pub unsigned_data_object_properties: Option<UnsignedDataObjectProperties>,
}
