use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Property {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<crate::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "Name")]
    pub name: crate::cct::TextType,
    #[serde(rename = "Value")]
    pub value: crate::cct::TextType,
}
