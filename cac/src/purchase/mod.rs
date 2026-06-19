use serde::{Deserialize, Serialize};

include!("receipt_line.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class for referencing an object to which a purchase relates, such as a subscription number,
/// telephone number, meter, vehicle, person, etc.
///
/// UBL Dictionary Entry Name: `Purchase Reference. Details`
///
/// Generated from XSD type `PurchaseReferenceType`.
pub struct PurchaseReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An Identifier for this purchase reference.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A description of this purchase reference.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
