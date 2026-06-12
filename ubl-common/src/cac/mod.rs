// UBL 2.5 Common Aggregate Components (CAC)
//
// Reusable complex structures (ABIEs) that compose CBC types into
// business-meaningful aggregates. These are the building blocks that
// document types assemble into complete business documents.

pub mod address;
pub mod allowance;
pub mod contact;
pub mod customer;
pub mod delivery;
pub mod document;
pub mod item;
pub mod party;
pub mod payment;
pub mod period;
pub mod price;
pub mod supplier;
pub mod tax;
pub mod totals;

// Re-export all
pub use address::*;
pub use allowance::*;
pub use contact::*;
pub use customer::*;
pub use delivery::*;
pub use document::*;
pub use item::*;
pub use party::*;
pub use payment::*;
pub use period::*;
pub use price::*;
pub use supplier::*;
pub use tax::*;
pub use totals::*;
