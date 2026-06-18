#[derive(Debug, Deserialize, Serialize)]
pub struct EndorserParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "RoleCode")]
    pub role_code: super::cct::CodeType,
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: super::cct::NumericType,
    #[serde(rename = "Party")]
    pub party: Party,
    #[serde(rename = "SignatoryContact")]
    pub signatory_contact: Contact,
}
