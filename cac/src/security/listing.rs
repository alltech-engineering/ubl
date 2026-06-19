#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a financial security listed on a regulated market.
///
/// UBL Dictionary Entry Name: `Security Listing. Details`
///
/// Generated from XSD type `SecurityListingType`.
pub struct SecurityListing {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the listed security, such as an ISIN.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A description of the listed security, such as the name or type of the instrument.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The name of the regulated market on which this security is listed.
    #[serde(rename = "MarketName")]
    pub market_name: cct::Text,
/// A code identifying the regulated market (e.g., MIC code as per ISO 10383).
    #[serde(default, rename = "MarketCode")]
    pub market_code: Option<cct::Code>,
}
