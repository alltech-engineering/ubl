use serde::{Deserialize, Serialize};


include!("listing.rs");
include!("measure.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to specify security clearance terms.
///
/// UBL Dictionary Entry Name: `Security Clearance Term. Details`
///
/// Generated from XSD type `SecurityClearanceTermType`.
pub struct SecurityClearanceTerm {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code signifying the security clearance requirement.
    #[serde(default, rename = "Code")]
    pub code: Option<cct::Code>,
/// A description of the security clearance requirement.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
