#[derive(Debug, Deserialize, Serialize)]
pub struct TransportEquipmentSeal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "SealIssuerTypeCode")]
    pub seal_issuer_type_code: Option<cct::Code>,
    #[serde(default, rename = "Condition")]
    pub condition: Option<cct::Text>,
    #[serde(default, rename = "SealStatusCode")]
    pub seal_status_code: Option<cct::Code>,
    #[serde(default, rename = "SealingPartyType")]
    pub sealing_party_type: Option<cct::Text>,
}
