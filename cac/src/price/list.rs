#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a price list.
///
/// UBL Dictionary Entry Name: `Price List. Details`
///
/// Generated from XSD type `PriceListType`.
pub struct PriceList {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this price list.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code signifying whether this price list is an original, copy, revision, or cancellation.
    #[serde(default, rename = "StatusCode")]
    pub status_code: Option<cct::Code>,
/// A period during which this price list is valid.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<crate::Period>,
/// The previous price list.
    #[serde(default, rename = "PreviousPriceList")]
    pub previous_price_list: Option<Box<PriceList>>,
}
