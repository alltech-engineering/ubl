#[derive(Debug, Deserialize, Serialize)]
/// Numeric information that is assigned or is determined by calculation, counting, or sequencing. It
/// does not require a unit of quantity or unit of measure.
///
/// UBL Dictionary Entry Name: `Numeric. Type`
///
/// Generated from XSD type `NumericType`.
pub struct Numeric {
/// (Deprecated) Whether the number is an integer, decimal, real number or percentage.
    #[serde(default, rename = "@format")]
    pub format: Option<String>,
    #[serde(rename = "$text")]
    pub content: f64,
}
