#[derive(Debug, Deserialize, Serialize)]
pub struct DsaKeyValue {
    #[serde(default, rename = "P")]
    pub p: Option<String>,
    #[serde(default, rename = "Q")]
    pub q: Option<String>,
    #[serde(default, rename = "G")]
    pub g: Option<String>,
    #[serde(rename = "Y")]
    pub y: String,
    #[serde(default, rename = "J")]
    pub j: Option<String>,
    #[serde(default, rename = "Seed")]
    pub seed: Option<String>,
    #[serde(default, rename = "PgenCounter")]
    pub pgen_counter: Option<String>,
}
