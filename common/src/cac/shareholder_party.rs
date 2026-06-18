#[derive(Debug, Deserialize, Serialize)]
pub struct ShareholderParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "PartecipationPercent")]
    pub partecipation_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "ParticipationPercent")]
    pub participation_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "Party")]
    pub party: Option<Box<Party>>,
}
