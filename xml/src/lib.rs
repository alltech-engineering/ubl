// UBL 2.5 XML Adapter
//
// Handles serialization/deserialization between Rust domain types
// and UBL 2.5 XML documents with proper namespace handling.
//
// Design:
//   - Domain crates (ubl-common, ubl-documents) are XML-free
//   - This crate wraps domain types with XML-specific serialization logic
//   - Uses quick-xml for high-performance XML I/O
//   - Handles cbc:, cac:, and document-level namespace prefixes

pub mod de;
pub mod error;
pub mod ns;
pub mod ser;

pub use de::from_str;
pub use error::{Error, Result};
pub use ser::to_string;
