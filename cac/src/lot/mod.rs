use serde::{Deserialize, Serialize};

include!("distribution.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct LotIdentification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "LotNumberID")]
    pub lot_number_id: Option<cct::Identifier>,
    #[serde(default, rename = "ExpiryDate")]
    pub expiry_date: Option<udt::DateTime>,
    #[serde(default, rename = "AdditionalItemProperty")]
    pub additional_item_property: Vec<crate::ItemProperty>,
}
