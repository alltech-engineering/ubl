#[derive(Debug, Deserialize, Serialize)]
pub struct OtherCertStatusRefsType {
    #[serde(default, rename = "OtherRef")]
    pub other_ref: Vec<Any>,
}
