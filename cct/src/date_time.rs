#[derive(Debug, Deserialize, Serialize)]
/// A particular point in the progression of time, together with relevant supplementary information.
///
/// UBL Dictionary Entry Name: `Date Time. Type`
///
/// Generated from XSD type `DateTimeType`.
pub struct DateTime {
    #[serde(default, rename = "@format")]
    pub format: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
