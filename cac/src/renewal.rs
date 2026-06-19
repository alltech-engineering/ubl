#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the renewal of a commercial arrangement, such as a contract or licence fee.
///
/// UBL Dictionary Entry Name: `Renewal. Details`
///
/// Generated from XSD type `RenewalType`.
pub struct Renewal {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The monetary amount of this renewal.
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
/// The period for which the arrangement is now valid
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
}
