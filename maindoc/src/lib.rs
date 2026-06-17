// UBL 2.5 Document Types
//
// Reference: https://docs.oasis-open.org/ubl/cs01-UBL-2.5/UBL-2.5.html
#![allow(ambiguous_glob_imports)]
#![allow(ambiguous_glob_reexports)]

pub mod billing;
pub mod catalogue;
pub mod despatch;
pub mod ordering;
pub mod quotation;
pub mod tendering;
pub mod transportation;

mod application_response;
pub use application_response::ApplicationResponse;

// P8: Remaining Documents — 34 types across 6 categories
pub mod customs;
pub mod directory;
pub mod inventory;
pub mod other;
pub mod status;
pub mod waste;
