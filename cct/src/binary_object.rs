#[derive(Debug, Deserialize, Serialize)]
/// A set of finite-length sequences of binary octets.
///
/// UBL Dictionary Entry Name: `Binary Object. Type`
///
/// Generated from XSD type `BinaryObjectType`.
pub struct BinaryObject {
/// (Deprecated) The format of the binary content.
    #[serde(default, rename = "@format")]
    pub format: Option<String>,
/// The mime type of the binary object.
    #[serde(default, rename = "@mimeCode")]
    pub mime_code: Option<String>,
/// (Deprecated) Specifies the decoding algorithm of the binary object.
    #[serde(default, rename = "@encodingCode")]
    pub encoding_code: Option<String>,
/// (Deprecated) The character set of the binary object if the mime type is text.
    #[serde(default, rename = "@characterSetCode")]
    pub character_set_code: Option<String>,
/// (Deprecated) The Uniform Resource Identifier that identifies where the binary object is located.
    #[serde(default, rename = "@uri")]
    pub uri: Option<String>,
/// (Deprecated) The filename of the binary object.
    #[serde(default, rename = "@filename")]
    pub filename: Option<String>,
    #[serde(rename = "$text")]
    pub content: String,
}
