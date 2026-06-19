#[derive(Debug, Deserialize, Serialize)]
pub struct ShareholderParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "PartecipationPercent")]
    pub partecipation_percent: Option<cct::Numeric>,
    #[serde(default, rename = "ParticipationPercent")]
    pub participation_percent: Option<cct::Numeric>,
    #[serde(default, rename = "Party")]
    pub party: Option<Box<Party>>,
}
