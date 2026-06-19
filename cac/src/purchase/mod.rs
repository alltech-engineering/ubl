use serde::{Deserialize, Serialize};

include!("receipt_line.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct PurchaseReference {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
