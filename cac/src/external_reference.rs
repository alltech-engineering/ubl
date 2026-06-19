#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an external object, such as a document stored at a remote location.
///
/// UBL Dictionary Entry Name: `External Reference. Details`
///
/// Generated from XSD type `ExternalReferenceType`.
pub struct ExternalReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The Uniform Resource Identifier (URI) that identifies the external object as an Internet resource.
    #[serde(default, rename = "URI")]
    pub uri: Option<cct::Identifier>,
/// A hash value for the externally stored object.
    #[serde(default, rename = "DocumentHash")]
    pub document_hash: Option<cct::Text>,
/// A hash algorithm used to calculate the hash value of the externally stored object.
    #[serde(default, rename = "HashAlgorithmMethod")]
    pub hash_algorithm_method: Option<cct::Text>,
/// The date on which availability of the resource can no longer be relied upon.
    #[serde(default, rename = "ExpiryDate")]
    pub expiry_date: Option<udt::DateTime>,
/// The time after which availability of the resource can no longer be relied upon.
    #[serde(default, rename = "ExpiryTime")]
    pub expiry_time: Option<udt::DateTime>,
/// A code signifying the mime type of the external object.
    #[serde(default, rename = "MimeCode")]
    pub mime_code: Option<cct::Code>,
/// A code signifying the format of the external object.
    #[serde(default, rename = "FormatCode")]
    pub format_code: Option<cct::Code>,
/// A code signifying the encoding/decoding algorithm used with the external object.
    #[serde(default, rename = "EncodingCode")]
    pub encoding_code: Option<cct::Code>,
/// A code signifying the character set of an external document.
    #[serde(default, rename = "CharacterSetCode")]
    pub character_set_code: Option<cct::Code>,
/// The file name of the external object.
    #[serde(default, rename = "FileName")]
    pub file_name: Option<cct::Text>,
/// Text describing the external object.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
