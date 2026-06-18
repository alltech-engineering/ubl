#[derive(Debug, Deserialize, Serialize)]
pub struct DataObjectFormat {
    #[serde(rename = "@ObjectReference")]
    pub object_reference: String,
    #[serde(default, rename = "Description")]
    pub description: Option<String>,
    #[serde(default, rename = "ObjectIdentifier")]
    pub object_identifier: Option<ObjectIdentifier>,
    #[serde(default, rename = "MimeType")]
    pub mime_type: Option<String>,
    #[serde(default, rename = "Encoding")]
    pub encoding: Option<String>,
}
