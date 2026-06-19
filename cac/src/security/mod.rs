use serde::{Deserialize, Serialize};


include!("listing.rs");
include!("measure.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityClearanceTerm {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "Code")]
    pub code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
