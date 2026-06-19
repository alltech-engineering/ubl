#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the criterion requirement response value.
///
/// UBL Dictionary Entry Name: `Response Value. Details`
///
/// Generated from XSD type `ResponseValueType`.
pub struct ResponseValue {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier to refer to the criterion requirement response value.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A description of the response value to the criterion requirement.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A text or name used as a reply to the criterion requirement.
    #[serde(default, rename = "Response")]
    pub response: Vec<cct::Text>,
/// An amount used as a reply to the criterion requirement.
    #[serde(default, rename = "ResponseAmount")]
    pub response_amount: Option<cct::Amount>,
/// A binary graphic, picture, sound or video object used as a reply to the criterion requirement.
    #[serde(default, rename = "ResponseBinaryObject")]
    pub response_binary_object: Option<cct::BinaryObject>,
/// A code used as a reply to the criterion requirement.
    #[serde(default, rename = "ResponseCode")]
    pub response_code: Option<cct::Code>,
/// A date used as a reply to the criterion requirement.
    #[serde(default, rename = "ResponseDate")]
    pub response_date: Option<udt::DateTime>,
/// An identifier used as a reply to the criterion requirement.
    #[serde(default, rename = "ResponseID")]
    pub response_id: Option<cct::Identifier>,
/// An indicator used as a reply to the criterion requirement.
    #[serde(default, rename = "ResponseIndicator")]
    pub response_indicator: Option<udt::Indicator>,
/// A measure used as a reply to the criterion requirement.
    #[serde(default, rename = "ResponseMeasure")]
    pub response_measure: Option<cct::Measure>,
/// A number, rate or percent used as a reply to the criterion requirement.
    #[serde(default, rename = "ResponseNumeric")]
    pub response_numeric: Option<cct::Numeric>,
/// A quantity used as a reply to the criterion requirement.
    #[serde(default, rename = "ResponseQuantity")]
    pub response_quantity: Option<cct::Quantity>,
/// A time used as a reply to the criterion requirement.
    #[serde(default, rename = "ResponseTime")]
    pub response_time: Option<udt::DateTime>,
/// A URI value used as a reply to the criterion requirement.
    #[serde(default, rename = "ResponseURI")]
    pub response_uri: Option<cct::Identifier>,
}
