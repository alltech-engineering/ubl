#[derive(Debug, Deserialize, Serialize)]
pub struct SignerRoleV2 {
    #[serde(default, rename = "ClaimedRoles")]
    pub claimed_roles: Option<ClaimedRolesListType>,
    #[serde(default, rename = "CertifiedRolesV2")]
    pub certified_roles_v2: Option<CertifiedRolesListTypeV2Type>,
    #[serde(default, rename = "SignedAssertions")]
    pub signed_assertions: Option<SignedAssertionsListType>,
}
