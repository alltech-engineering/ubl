use serde::{Deserialize, Serialize};
pub type AmountType = cct::Amount;
pub type BinaryObject = cct::BinaryObject;
pub type Code = cct::Code;
#[derive(Debug, Deserialize, Serialize)]
pub struct DateTime {
    #[serde(default, rename = "$text")]
    pub content: String,
}
pub type DateType = DateTime;
pub type GraphicType = cct::BinaryObject;
pub type Identifier = cct::Identifier;
#[derive(Debug, Deserialize, Serialize)]
pub struct Indicator {
    #[serde(rename = "$text")]
    pub content: bool,
}
pub type Measure = cct::Measure;
pub type NameType = cct::Text;
pub type Numeric = cct::Numeric;
pub type PercentType = cct::Numeric;
pub type PictureType = cct::BinaryObject;
pub type Quantity = cct::Quantity;
pub type RateType = cct::Numeric;
pub type SoundType = cct::BinaryObject;
pub type Text = cct::Text;
pub type TimeType = DateTime;
pub type ValueType = cct::Numeric;
pub type VideoType = cct::BinaryObject;
