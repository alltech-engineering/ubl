// UBL Binary Object types — embedded binary data with MIME type metadata.

use serde::{Deserialize, Serialize};

/// A binary object — typically an embedded file (PDF, image, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryObject {
    /// Base64-encoded or raw bytes
    pub value: Vec<u8>,
    /// MIME type (e.g., "application/pdf", "image/png")
    pub mime_code: String,
    /// Optional filename
    pub filename: Option<String>,
    /// Optional character set code
    pub character_set_code: Option<String>,
    /// Optional encoding code
    pub encoding_code: Option<String>,
    /// Optional URI to external content
    pub uri: Option<String>,
}

impl BinaryObject {
    pub fn new(value: Vec<u8>, mime_code: impl Into<String>) -> Self {
        Self {
            value,
            mime_code: mime_code.into(),
            filename: None,
            character_set_code: None,
            encoding_code: None,
            uri: None,
        }
    }
}

macro_rules! define_binary_object {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $name(pub BinaryObject);
        impl $name {
            pub fn new(value: Vec<u8>, mime_code: impl Into<String>) -> Self {
                Self(BinaryObject::new(value, mime_code))
            }
        }
    };
}

define_binary_object!(BinaryObjectType, "A generic embedded binary object.");
define_binary_object!(EmbeddedDocumentBinaryObject, "An embedded document as binary.");
define_binary_object!(SignatureBinaryObject, "A digital signature as binary data.");

