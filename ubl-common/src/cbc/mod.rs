// UBL 2.5 Common Basic Components (CBC)
//
// Each CBC type maps to a named UBL XML element with a specific semantic
// meaning. Types are organised by representation term (Amount, Code, Date, etc.)
//
// All types derive: Debug, Clone, PartialEq, Eq/Hash (where applicable),
// Serialize, Deserialize.

pub mod amount;
pub mod binary;
pub mod code;
pub mod date;
pub mod identifier;
pub mod indicator;
pub mod measure;
pub mod numeric;
pub mod quantity;
pub mod text;

// Re-export all types at the cbc module level
pub use amount::*;
pub use binary::*;
pub use code::*;
pub use date::*;
pub use identifier::*;
pub use indicator::*;
pub use measure::*;
pub use numeric::*;
pub use quantity::*;
pub use text::*;
