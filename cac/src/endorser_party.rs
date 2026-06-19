#[derive(Debug, Deserialize, Serialize)]
pub struct EndorserParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "RoleCode")]
    pub role_code: cct::Code,
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: cct::Numeric,
    #[serde(rename = "Party")]
    pub party: Party,
    #[serde(rename = "SignatoryContact")]
    pub signatory_contact: Contact,
}
