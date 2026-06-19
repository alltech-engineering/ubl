use serde::{Deserialize, Serialize};

include!("requirement.rs");
include!("to_ship_activity_record.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct ShipStoreArticle {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
    #[serde(default, rename = "OfficialUse")]
    pub official_use: Vec<cct::Text>,
    #[serde(default, rename = "Stowage")]
    pub stowage: Option<crate::Stowage>,
}
