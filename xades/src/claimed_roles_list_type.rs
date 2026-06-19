#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimedRolesListType {
    #[serde(default, rename = "ClaimedRole")]
    pub claimed_role: Vec<Any>,
}
