#[derive(Debug, Deserialize, Serialize)]
pub struct SignerRole {
    #[serde(default, rename = "ClaimedRoles")]
    pub claimed_roles: Option<ClaimedRolesListType>,
    #[serde(default, rename = "CertifiedRoles")]
    pub certified_roles: Option<CertifiedRolesListType>,
}
