#[derive(Debug, Deserialize, Serialize)]
pub struct CertifiedRolesListType {
    #[serde(default, rename = "CertifiedRole")]
    pub certified_role: Vec<EncapsulatedPkiData>,
}
