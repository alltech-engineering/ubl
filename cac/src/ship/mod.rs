use serde::{Deserialize, Serialize};

include!("requirement.rs");
include!("to_ship_activity_record.rs");

#[derive(Debug, Deserialize, Serialize)]
/// An article in the ship's stores during a shipment stage.
///
/// UBL Dictionary Entry Name: `Ship Store Article. Details`
///
/// Generated from XSD type `ShipStoreArticleType`.
pub struct ShipStoreArticle {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An indentifier for this ship store article.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A name for this ship store article.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// The quantity of this ship store article.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// A text describing the official use of this ship store article.
    #[serde(default, rename = "OfficialUse")]
    pub official_use: Vec<cct::Text>,
/// The stowage or location on board of this ship store article.
    #[serde(default, rename = "Stowage")]
    pub stowage: Option<crate::Stowage>,
}
