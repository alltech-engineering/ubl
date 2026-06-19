use serde::{Deserialize, Serialize};

include!("distribution.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class for defining a lot identifier (the identifier of a set of item instances that would be used
/// in case of a recall of that item).
///
/// UBL Dictionary Entry Name: `Lot Identification. Details`
///
/// Generated from XSD type `LotIdentificationType`.
pub struct LotIdentification {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the lot.
    #[serde(default, rename = "LotNumberID")]
    pub lot_number_id: Option<cct::Identifier>,
/// The expiry date of the lot.
    #[serde(default, rename = "ExpiryDate")]
    pub expiry_date: Option<udt::DateTime>,
/// An additional property of the lot.
    #[serde(default, rename = "AdditionalItemProperty")]
    pub additional_item_property: Vec<crate::ItemProperty>,
}
