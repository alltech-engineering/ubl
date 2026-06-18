#[derive(Debug, Deserialize, Serialize)]
pub struct CertifiedRolesListTypeV2Type {
    #[serde(default, rename = "CertifiedRole")]
    pub certified_role: Vec<CertifiedRoleTypeV2Type>,
}
