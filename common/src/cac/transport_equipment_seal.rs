#[derive(Debug, Deserialize, Serialize)]
pub struct TransportEquipmentSeal {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "SealIssuerTypeCode")]
    pub seal_issuer_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Condition")]
    pub condition: Option<super::cct::TextType>,
    #[serde(default, rename = "SealStatusCode")]
    pub seal_status_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "SealingPartyType")]
    pub sealing_party_type: Option<super::cct::TextType>,
}
