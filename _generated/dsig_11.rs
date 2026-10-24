use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct CharTwoFieldParamsType {
    #[serde(rename = "M")]
    pub m: ::core::num::NonZeroUsize,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CurveType {
    #[serde(rename = "A")]
    pub a: ::std::string::String,
    #[serde(rename = "B")]
    pub b: ::std::string::String,
}
pub type DerEncodedKeyValue = DerEncodedKeyValueType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DerEncodedKeyValueType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$text")]
    pub content: ::std::string::String,
}
pub type EcKeyValue = EcKeyValueType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EcKeyValueType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<EcKeyValueTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum EcKeyValueTypeContent {
    #[serde(rename = "ECParameters")]
    EcParameters(EcParametersType),
    #[serde(rename = "NamedCurve")]
    NamedCurve(NamedCurveType),
    #[serde(rename = "PublicKey")]
    PublicKey(::std::string::String),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EcParametersType {
    #[serde(rename = "FieldID")]
    pub field_id: FieldIdType,
    #[serde(rename = "Curve")]
    pub curve: CurveType,
    #[serde(rename = "Base")]
    pub base: ::std::string::String,
    #[serde(rename = "Order")]
    pub order: ::std::string::String,
    #[serde(default, rename = "CoFactor")]
    pub co_factor: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "ValidationData")]
    pub validation_data: ::core::option::Option<EcValidationDataType>,
}
pub type EcPointType = ::std::string::String;
#[derive(Debug, Deserialize, Serialize)]
pub struct EcValidationDataType {
    #[serde(rename = "@hashAlgorithm")]
    pub hash_algorithm: ::std::string::String,
    #[serde(rename = "seed")]
    pub seed: ::std::string::String,
}
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
pub type GnB = CharTwoFieldParamsType;
pub type KeyInfoReference = KeyInfoReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyInfoReferenceType {
    #[serde(rename = "@URI")]
    pub uri: ::std::string::String,
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct NamedCurveType {
    #[serde(rename = "@URI")]
    pub uri: ::std::string::String,
}
pub type PnB = PnBFieldParamsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PnBFieldParamsType {
    #[serde(rename = "M")]
    pub m: ::core::num::NonZeroUsize,
    #[serde(rename = "K1")]
    pub k1: ::core::num::NonZeroUsize,
    #[serde(rename = "K2")]
    pub k2: ::core::num::NonZeroUsize,
    #[serde(rename = "K3")]
    pub k3: ::core::num::NonZeroUsize,
}
pub type Prime = PrimeFieldParamsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PrimeFieldParamsType {
    #[serde(rename = "P")]
    pub p: ::std::string::String,
}
pub type TnB = TnBFieldParamsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TnBFieldParamsType {
    #[serde(rename = "M")]
    pub m: ::core::num::NonZeroUsize,
    #[serde(rename = "K")]
    pub k: ::core::num::NonZeroUsize,
}
pub type X509Digest = X509DigestType;
#[derive(Debug, Deserialize, Serialize)]
pub struct X509DigestType {
    #[serde(rename = "@Algorithm")]
    pub algorithm: ::std::string::String,
    #[serde(rename = "$text")]
    pub content: ::std::string::String,
}
