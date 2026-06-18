mod data_line;
pub use data_line::*;

mod property;
pub use property::*;

pub type FinalLocation = crate::cac::Location;
pub type OriginLocation = crate::cac::Location;
pub type Period = crate::cac::Period;

//include!("data_line.rs");
//include!("property.rs");
