use serde::{Deserialize, Serialize};


include!("consumption.rs");
include!("property.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe fuel metering.
///
/// UBL Dictionary Entry Name: `Fuel Metering. Details`
///
/// Generated from XSD type `FuelMeteringType`.
pub struct FuelMetering {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the type of fuel metering.
    #[serde(rename = "TypeID")]
    pub type_id: cct::Identifier,
/// The value of this fuel metering.
    #[serde(rename = "Value")]
    pub value: cct::Text,
}
