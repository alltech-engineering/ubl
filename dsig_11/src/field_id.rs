#[derive(Debug, Deserialize, Serialize)]
pub enum FieldId {
    #[serde(rename = "Prime")]
    Prime(PrimeFieldParams),
    #[serde(rename = "TnB")]
    TnB(TnBFieldParams),
    #[serde(rename = "PnB")]
    PnB(PnBFieldParams),
    #[serde(rename = "GnB")]
    GnB(CharTwoFieldParams),
    #[serde(rename = "any44")]
    Any(String),
}
