// UBL 2.5 Common Aggregate Components (CAC)
//
// Reusable complex structures (ABIEs) that compose CBC types into
// business-meaningful aggregates.

pub mod address;
pub mod allowance;
pub mod contact;
pub mod customer;
pub mod delivery;
pub mod dimension;
pub mod document;
pub mod exchange_rate;
pub mod item;
pub mod line;
pub mod order_reference;
pub mod party;
pub mod payment;
pub mod period;
pub mod price;
pub mod response;
pub mod supplier;
pub mod tax;
pub mod totals;
pub mod transport;

// Re-export all
pub use address::*;
pub use allowance::*;
pub use contact::*;
pub use customer::*;
pub use delivery::*;
pub use dimension::*;
pub use document::*;
pub use exchange_rate::*;
pub use item::*;
pub use line::*;
pub use order_reference::*;
pub use party::*;
pub use payment::*;
pub use period::*;
pub use price::*;
pub use response::*;
pub use supplier::*;
pub use tax::*;
pub use totals::*;
pub use transport::*;
