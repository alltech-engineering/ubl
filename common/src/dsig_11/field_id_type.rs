#[derive(Debug, Deserialize, Serialize)]
pub enum FieldIdType {
    #[serde(rename = "Prime")]
    Prime(PrimeFieldParamsType),
    #[serde(rename = "TnB")]
    TnB(TnBFieldParamsType),
    #[serde(rename = "PnB")]
    PnB(PnBFieldParamsType),
    #[serde(rename = "GnB")]
    GnB(CharTwoFieldParamsType),
    #[serde(rename = "any44")]
    Any(String),
}
