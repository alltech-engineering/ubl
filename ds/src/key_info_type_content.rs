#[derive(Debug, Deserialize, Serialize)]
pub enum KeyInfoTypeContent {
    #[serde(rename = "KeyName")]
    KeyName(String),
    #[serde(rename = "KeyValue")]
    KeyValue(KeyValue),
    #[serde(rename = "RetrievalMethod")]
    RetrievalMethod(RetrievalMethod),
    #[serde(rename = "X509Data")]
    X509Data(X509Data),
    #[serde(rename = "PGPData")]
    PgpData(PgpData),
    #[serde(rename = "SPKIData")]
    SpkiData(SpkiData),
    #[serde(rename = "MgmtData")]
    MgmtData(String),
    #[serde(rename = "any15")]
    Any(String),
    #[serde(rename = "$text")]
    Text(String),
}
