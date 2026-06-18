#[derive(Debug, Deserialize, Serialize)]
pub struct Attestation {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "AcceptanceIndicator")]
    pub acceptance_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<Party>,
    #[serde(default, rename = "AttestationLine")]
    pub attestation_line: Vec<AttestationLine>,
}
