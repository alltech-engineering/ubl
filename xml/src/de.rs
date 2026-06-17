// UBL XML Deserializer — stub
//
// XML deserialization is more complex than serialization due to
// namespace handling and optional elements. Will be implemented
// as needed for specific document types.

use crate::error::Result;

/// Parse a UBL XML string into a domain type.
/// Not yet implemented — returns an error.
pub fn from_str<T>(_xml: &str) -> Result<T> {
    Err(crate::error::Error::UnexpectedStructure(
        "XML deserialization not yet implemented".into(),
    ))
}
