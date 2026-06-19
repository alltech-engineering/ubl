use serde::{Deserialize, Serialize};
use std::num::{NonZeroIsize, NonZeroUsize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Entities(pub Vec<String>);

pub type Entity = String;
pub type Id = String;
pub type Idref = String;
pub type Idrefs = Entities;
pub type NcName = String;
pub type Nmtoken = String;
pub type Nmtokens = Entities;
pub type Notation = String;
pub type Name = String;
pub type QName = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct AnySimple {
    #[serde(default, rename = "@type")]
    pub type_: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}

pub type AnyUri = String;
pub type Boolean = bool;
pub type Byte = i8;
pub type Date = String;
pub type DateTime = String;
pub type Decimal = f64;
pub type Double = f64;
pub type Duration = String;
pub type Float = f32;
pub type GDay = String;
pub type GMonth = String;
pub type GMonthDay = String;
pub type GYearType = String;
pub type GYearMonth = String;
pub type Int = i32;
pub type Integer = i32;
pub type Language = String;
pub type Long = i64;
pub type NegativeInteger = NonZeroIsize;
pub type NonNegativeInteger = usize;
pub type NonPositiveInteger = isize;
pub type NormalizedString = String;
pub type PositiveInteger = NonZeroUsize;
pub type Short = i16;
// pub type String = String;
pub type Time = String;
pub type Token = String;
pub type UnsignedByte = u8;
pub type UnsignedInt = u32;
pub type UnsignedLong = u64;
pub type UnsignedShort = u16;
