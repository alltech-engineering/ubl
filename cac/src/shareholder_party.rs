#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a Party that owns shares or equity.
///
/// UBL Dictionary Entry Name: `Shareholder Party. Details`
///
/// Generated from XSD type `ShareholderPartyType`.
pub struct ShareholderParty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// (Deprecated) The shareholder participation, expressed as a percentage.
    #[serde(default, rename = "PartecipationPercent")]
    pub partecipation_percent: Option<cct::Numeric>,
/// The percentage of shares or equity owned by this Party.
    #[serde(default, rename = "ParticipationPercent")]
    pub participation_percent: Option<cct::Numeric>,
/// The Party that owns shares or equity.
    #[serde(default, rename = "Party")]
    pub party: Option<Box<Party>>,
}
