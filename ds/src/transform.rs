#[derive(Debug, Deserialize, Serialize)]
pub struct Transform {
    #[serde(rename = "@Algorithm")]
    pub algorithm: String,
    #[serde(default, rename = "$value")]
    pub content: Vec<TransformTypeContent>,
}
