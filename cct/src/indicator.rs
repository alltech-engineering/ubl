#[derive(Debug, Deserialize, Serialize)]
/// A list of two mutually exclusive Boolean values that express the only possible states of a property.
///
/// UBL Dictionary Entry Name: `Indicator. Type`
///
/// Generated from XSD type `IndicatorType`.
pub struct Indicator {
    #[serde(default, rename = "@format")]
    pub format: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
