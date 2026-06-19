#[derive(Debug, Deserialize, Serialize)]
/// A simplified version of the Price class intended for applications such as telephone billing.
///
/// UBL Dictionary Entry Name: `Unstructured Price. Details`
///
/// Generated from XSD type `UnstructuredPriceType`.
pub struct UnstructuredPrice {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The price amount.
    #[serde(default, rename = "PriceAmount")]
    pub price_amount: Option<cct::Amount>,
/// The price amount inclusive of all taxes.
    #[serde(default, rename = "TaxInclusivePriceAmount")]
    pub tax_inclusive_price_amount: Option<cct::Amount>,
/// The usage time upon which the price is based.
    #[serde(default, rename = "TimeAmount")]
    pub time_amount: Option<cct::Text>,
}
