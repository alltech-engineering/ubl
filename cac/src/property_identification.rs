#[derive(Debug, Deserialize, Serialize)]
pub struct PropertyIdentification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "IssuerScopeID")]
    pub issuer_scope_id: Option<cct::Identifier>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<Party>,
}
