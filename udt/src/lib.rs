use serde::{Deserialize, Serialize};
pub type Amount = cct::Amount;
pub type BinaryObject = cct::BinaryObject;
pub type Code = cct::Code;
#[derive(Debug, Deserialize, Serialize)]
/// A particular point in the progression of time, together with relevant supplementary information.
///
/// UBL Dictionary Entry Name: `Date Time. Type`
///
/// Generated from XSD type `DateTimeType`.
pub struct DateTime {
    #[serde(default, rename = "$text")]
    pub content: String,
}
pub type Date = DateTime;
pub type Graphic = cct::BinaryObject;
pub type Identifier = cct::Identifier;
#[derive(Debug, Deserialize, Serialize)]
/// A list of two mutually exclusive Boolean values that express the only possible states of a property.
///
/// UBL Dictionary Entry Name: `Indicator. Type`
///
/// Generated from XSD type `IndicatorType`.
pub struct Indicator {
    #[serde(rename = "$text")]
    pub content: bool,
}
pub type Measure = cct::Measure;
pub type Name = cct::Text;
pub type Numeric = cct::Numeric;
pub type Percent = cct::Numeric;
pub type Picture = cct::BinaryObject;
pub type Quantity = cct::Quantity;
pub type Rate = cct::Numeric;
pub type Sound = cct::BinaryObject;
pub type Text = cct::Text;
pub type Time = DateTime;
pub type Value = cct::Numeric;
pub type Video = cct::BinaryObject;
