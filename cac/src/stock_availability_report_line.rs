#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Stock Availability Report describing the availability of an item of
/// sale.
///
/// UBL Dictionary Entry Name: `Stock Availability Report Line. Details`
///
/// Generated from XSD type `StockAvailabilityReportLineType`.
pub struct StockAvailabilityReportLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this stock availability line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The quantity of the item currently in stock.
    #[serde(rename = "Quantity")]
    pub quantity: cct::Quantity,
/// The monetary value of the quantity of the item currently in stock.
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: Option<cct::Amount>,
/// The date from which the item will be available. A date identical to or earlier than the IssueDate of
/// the Stock Availability Report means that the item is available now
    #[serde(default, rename = "AvailabilityDate")]
    pub availability_date: Option<udt::DateTime>,
/// A code signifying the level of availability of the item.
    #[serde(default, rename = "AvailabilityStatusCode")]
    pub availability_status_code: Option<cct::Code>,
/// The item associated with this stock availability report line.
    #[serde(rename = "Item")]
    pub item: Item,
}
