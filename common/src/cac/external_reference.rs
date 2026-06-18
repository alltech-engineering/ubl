#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "URI")]
    pub uri: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DocumentHash")]
    pub document_hash: Option<super::cct::TextType>,
    #[serde(default, rename = "HashAlgorithmMethod")]
    pub hash_algorithm_method: Option<super::cct::TextType>,
    #[serde(default, rename = "ExpiryDate")]
    pub expiry_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ExpiryTime")]
    pub expiry_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "MimeCode")]
    pub mime_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "FormatCode")]
    pub format_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "EncodingCode")]
    pub encoding_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CharacterSetCode")]
    pub character_set_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "FileName")]
    pub file_name: Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
}
