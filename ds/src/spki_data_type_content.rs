#[derive(Debug, Deserialize, Serialize)]
pub struct SpkiDataTypeContent {
    #[serde(rename = "SPKISexp")]
    pub spki_sexp: String,
    #[serde(default, rename = "any29")]
    pub any: Option<String>,
}
