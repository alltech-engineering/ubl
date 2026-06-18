#[derive(Debug, Deserialize, Serialize)]
pub struct EconomicOperatorParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "QualifyingParty")]
    pub qualifying_party: Vec<QualifyingParty>,
    #[serde(default, rename = "EconomicOperatorRole")]
    pub economic_operator_role: Vec<EconomicOperatorRole>,
    #[serde(rename = "Party")]
    pub party: Party,
}
