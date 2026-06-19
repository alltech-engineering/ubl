#[derive(Debug, Deserialize, Serialize)]
pub struct WinningParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "Rank")]
    pub rank: Option<cct::Text>,
    #[serde(rename = "Party")]
    pub party: Party,
}
