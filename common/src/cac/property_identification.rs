#[derive(Debug, Deserialize, Serialize)]
pub struct PropertyIdentification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "IssuerScopeID")]
    pub issuer_scope_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<Party>,
}
