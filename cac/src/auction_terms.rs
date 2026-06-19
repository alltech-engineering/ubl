#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the terms to be fulfilled by tenderers if an auction is to be executed before
/// the awarding of a tender.
///
/// UBL Dictionary Entry Name: `Auction Terms. Details`
///
/// Generated from XSD type `AuctionTermsType`.
pub struct AuctionTerms {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// Indicates whether an electronic auction will be used before the awarding of a contract (true) or not
/// (false).
    #[serde(default, rename = "AuctionConstraintIndicator")]
    pub auction_constraint_indicator: Option<udt::Indicator>,
/// Text describing a justification for the use of an auction in awarding the tender.
    #[serde(default, rename = "JustificationDescription")]
    pub justification_description: Vec<cct::Text>,
/// Text for tenderers describing terms governing the auction.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Text describing the auction process.
    #[serde(default, rename = "ProcessDescription")]
    pub process_description: Vec<cct::Text>,
/// Text describing the conditions under which the tenderers will be able to bid as part of the auction.
    #[serde(default, rename = "ConditionsDescription")]
    pub conditions_description: Vec<cct::Text>,
/// Text describing an electronic device used for the auction, including associated connectivity
/// specifications.
    #[serde(default, rename = "ElectronicDeviceDescription")]
    pub electronic_device_description: Vec<cct::Text>,
/// The Uniform Resource Identifier (URI) of the electronic device used for the auction.
    #[serde(default, rename = "AuctionURI")]
    pub auction_uri: Option<cct::Identifier>,
}
