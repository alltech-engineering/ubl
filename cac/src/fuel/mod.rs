use serde::{Deserialize, Serialize};


include!("consumption.rs");
include!("property.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct FuelMetering {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "TypeID")]
    pub type_id: cct::Identifier,
    #[serde(rename = "Value")]
    pub value: cct::Text,
}
