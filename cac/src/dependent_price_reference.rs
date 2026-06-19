#[derive(Debug, Deserialize, Serialize)]
/// A class to define the price of an item as a percentage of the price of a different item.
///
/// UBL Dictionary Entry Name: `Dependent Price Reference. Details`
///
/// Generated from XSD type `DependentPriceReferenceType`.
pub struct DependentPriceReference {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The percentage by which the price of the different item is multiplied to calculate the price of the
/// item.
    #[serde(default, rename = "Percent")]
    pub percent: Option<cct::Numeric>,
/// The reference location for this dependent price reference.
    #[serde(default, rename = "LocationAddress")]
    pub location_address: Option<Address>,
/// A reference to a line that the price is depended of.
    #[serde(default, rename = "DependentLineReference")]
    pub dependent_line_reference: Option<LineReference>,
}
