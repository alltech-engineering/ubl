#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "URI")]
    pub uri: Option<cct::Identifier>,
    #[serde(default, rename = "DocumentHash")]
    pub document_hash: Option<cct::Text>,
    #[serde(default, rename = "HashAlgorithmMethod")]
    pub hash_algorithm_method: Option<cct::Text>,
    #[serde(default, rename = "ExpiryDate")]
    pub expiry_date: Option<udt::DateTime>,
    #[serde(default, rename = "ExpiryTime")]
    pub expiry_time: Option<udt::DateTime>,
    #[serde(default, rename = "MimeCode")]
    pub mime_code: Option<cct::Code>,
    #[serde(default, rename = "FormatCode")]
    pub format_code: Option<cct::Code>,
    #[serde(default, rename = "EncodingCode")]
    pub encoding_code: Option<cct::Code>,
    #[serde(default, rename = "CharacterSetCode")]
    pub character_set_code: Option<cct::Code>,
    #[serde(default, rename = "FileName")]
    pub file_name: Option<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
