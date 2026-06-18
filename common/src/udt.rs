use serde::{Deserialize, Serialize};
pub type AmountType = super::cct::AmountType;
pub type BinaryObjectType = super::cct::BinaryObjectType;
pub type CodeType = super::cct::CodeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DateTimeType {
    #[serde(default, rename = "$text")]
    pub content: String,
}
pub type DateType = DateTimeType;
pub type GraphicType = super::cct::BinaryObjectType;
pub type IdentifierType = super::cct::IdentifierType;
#[derive(Debug, Deserialize, Serialize)]
pub struct IndicatorType {
    #[serde(rename = "$text")]
    pub content: bool,
}
pub type MeasureType = super::cct::MeasureType;
pub type NameType = super::cct::TextType;
pub type NumericType = super::cct::NumericType;
pub type PercentType = super::cct::NumericType;
pub type PictureType = super::cct::BinaryObjectType;
pub type QuantityType = super::cct::QuantityType;
pub type RateType = super::cct::NumericType;
pub type SoundType = super::cct::BinaryObjectType;
pub type TextType = super::cct::TextType;
pub type TimeType = DateTimeType;
pub type ValueType = super::cct::NumericType;
pub type VideoType = super::cct::BinaryObjectType;
