#[derive(Debug, Deserialize, Serialize)]
/// A party that is identified as the awarded by a tender result.
///
/// UBL Dictionary Entry Name: `Winning Party. Details`
///
/// Generated from XSD type `WinningPartyType`.
pub struct WinningParty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// Indicates the rank obtained in the award.
    #[serde(default, rename = "Rank")]
    pub rank: Option<cct::Text>,
/// Information about an organization, sub-organization, or individual fulfilling a role in a business
/// process.
    #[serde(rename = "Party")]
    pub party: Party,
}
