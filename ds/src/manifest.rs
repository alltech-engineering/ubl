#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "Reference")]
    pub reference: Vec<Reference>,
}
