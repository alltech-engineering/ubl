use std::num::{NonZeroUsize, NonZeroIsize};
use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EntitiesType(pub Vec<String>);
pub type EntityType = String;
pub type IdType = String;
pub type IdrefType = String;
pub type IdrefsType = EntitiesType;
pub type NcNameType = String;
pub type NmtokenType = String;
pub type NmtokensType = EntitiesType;
pub type NotationType = String;
pub type NameType = String;
pub type QNameType = String;
#[derive(Debug, Deserialize, Serialize)]
pub struct AnySimpleType {
    #[serde(default, rename = "@type")]
    pub type_: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
pub type AnyUriType = String;
pub type BooleanType = bool;
pub type ByteType = i8;
pub type DateType = String;
pub type DateTimeType = String;
pub type DecimalType = f64;
pub type DoubleType = f64;
pub type DurationType = String;
pub type FloatType = f32;
pub type GDayType = String;
pub type GMonthType = String;
pub type GMonthDayType = String;
pub type GYearType = String;
pub type GYearMonthType = String;
pub type IntType = i32;
pub type IntegerType = i32;
pub type Language = String;
pub type LongType = i64;
pub type NegativeIntegerType = NonZeroIsize;
pub type NonNegativeIntegerType = usize;
pub type NonPositiveIntegerType = isize;
pub type NormalizedStringType = String;
pub type PositiveIntegerType = NonZeroUsize;
pub type ShortType = i16;
pub type StringType = String;
pub type TimeType = String;
pub type TokenType = String;
pub type UnsignedByteType = u8;
pub type UnsignedIntType = u32;
pub type UnsignedLongType = u64;
pub type UnsignedShortType = u16;
