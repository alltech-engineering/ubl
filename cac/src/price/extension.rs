#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a price extension, calculated by multiplying the price per unit by the quantity
/// of items.
///
/// UBL Dictionary Entry Name: `Price Extension. Details`
///
/// Generated from XSD type `PriceExtensionType`.
pub struct PriceExtension {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The amount of this price extension.
    #[serde(rename = "Amount")]
    pub amount: cct::Amount,
/// The amount of this price extension inclusive of all taxes.
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: Option<cct::Amount>,
/// A total amount of taxes of a particular kind applicable to this price extension.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
}
