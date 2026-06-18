use serde::{Deserialize, Serialize};
pub type CanonicalizationMethod = CanonicalizationMethodType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CanonicalizationMethodType {
    #[serde(rename = "@Algorithm")]
    pub algorithm: ::std::string::String,
    #[serde(default, rename = "$text")]
    pub text_before: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "any5")]
    pub any: ::std::vec::Vec<String>,
}
pub type CryptoBinaryType = ::std::string::String;
pub type DsaKeyValue = DsaKeyValueType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DsaKeyValueType {
    #[serde(default, rename = "P")]
    pub p: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Q")]
    pub q: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "G")]
    pub g: ::core::option::Option<::std::string::String>,
    #[serde(rename = "Y")]
    pub y: ::std::string::String,
    #[serde(default, rename = "J")]
    pub j: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Seed")]
    pub seed: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "PgenCounter")]
    pub pgen_counter: ::core::option::Option<::std::string::String>,
}
pub type DigestMethod = DigestMethodType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DigestMethodType {
    #[serde(rename = "@Algorithm")]
    pub algorithm: ::std::string::String,
    #[serde(default, rename = "$text")]
    pub text_before: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "any13")]
    pub any: ::std::vec::Vec<String>,
}
pub type DigestValue = ::std::string::String;
pub type DigestValueType = ::std::string::String;
pub type HmacOutputLengthType = ::core::primitive::i32;
pub type KeyInfo = KeyInfoType;
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyInfoType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<KeyInfoTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum KeyInfoTypeContent {
    #[serde(rename = "KeyName")]
    KeyName(::std::string::String),
    #[serde(rename = "KeyValue")]
    KeyValue(KeyValueType),
    #[serde(rename = "RetrievalMethod")]
    RetrievalMethod(RetrievalMethodType),
    #[serde(rename = "X509Data")]
    X509Data(X509DataType),
    #[serde(rename = "PGPData")]
    PgpData(PgpDataType),
    #[serde(rename = "SPKIData")]
    SpkiData(SpkiDataType),
    #[serde(rename = "MgmtData")]
    MgmtData(::std::string::String),
    #[serde(rename = "any15")]
    Any(String),
    #[serde(rename = "$text")]
    Text(::std::string::String),
}
pub type KeyName = ::std::string::String;
pub type KeyValue = KeyValueType;
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyValueType {
    #[serde(default, rename = "$text")]
    pub text_before: ::core::option::Option<::std::string::String>,
    #[serde(rename = "content")]
    pub content: KeyValueTypeContent,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum KeyValueTypeContent {
    #[serde(rename = "DSAKeyValue")]
    DsaKeyValue(DsaKeyValueType),
    #[serde(rename = "RSAKeyValue")]
    RsaKeyValue(RsaKeyValueType),
    #[serde(rename = "any17")]
    Any(String),
}
pub type Manifest = ManifestType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Reference")]
    pub reference: ::std::vec::Vec<ReferenceType>,
}
pub type MgmtData = ::std::string::String;
pub type Object = ObjectType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@MimeType")]
    pub mime_type: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@Encoding")]
    pub encoding: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "$value")]
    pub content: ::std::vec::Vec<ObjectTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectTypeContent {
    #[serde(default, rename = "$text")]
    pub text_before: ::core::option::Option<::std::string::String>,
    #[serde(rename = "any31")]
    pub any: String,
    #[serde(default, rename = "$text")]
    pub text_after_any_31: ::core::option::Option<::std::string::String>,
}
pub type PgpData = PgpDataType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PgpDataType {
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<PgpDataTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum PgpDataTypeContent {
    #[serde(rename = "PGPKeyID")]
    PgpKeyId(::std::string::String),
    #[serde(rename = "PGPKeyPacket")]
    PgpKeyPacket(::std::string::String),
    #[serde(rename = "any25")]
    Any(String),
    #[serde(rename = "any27")]
    Any2(String),
}
pub type RsaKeyValue = RsaKeyValueType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RsaKeyValueType {
    #[serde(rename = "Modulus")]
    pub modulus: ::std::string::String,
    #[serde(rename = "Exponent")]
    pub exponent: ::std::string::String,
}
pub type Reference = ReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ReferenceType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@URI")]
    pub uri: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@Type")]
    pub type_: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Transforms")]
    pub transforms: ::core::option::Option<TransformsType>,
    #[serde(rename = "DigestMethod")]
    pub digest_method: DigestMethodType,
    #[serde(rename = "DigestValue")]
    pub digest_value: ::std::string::String,
}
pub type RetrievalMethod = RetrievalMethodType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RetrievalMethodType {
    #[serde(default, rename = "@URI")]
    pub uri: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@Type")]
    pub type_: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Transforms")]
    pub transforms: ::core::option::Option<TransformsType>,
}
pub type SpkiData = SpkiDataType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SpkiDataType {
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<SpkiDataTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SpkiDataTypeContent {
    #[serde(rename = "SPKISexp")]
    pub spki_sexp: ::std::string::String,
    #[serde(default, rename = "any29")]
    pub any: ::core::option::Option<String>,
}
pub type Signature = SignatureType;
pub type SignatureMethod = SignatureMethodType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureMethodType {
    #[serde(rename = "@Algorithm")]
    pub algorithm: ::std::string::String,
    #[serde(default, rename = "$text")]
    pub text_before: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "HMACOutputLength")]
    pub hmac_output_length: ::core::option::Option<::core::primitive::i32>,
    #[serde(default, rename = "any7")]
    pub any: ::std::vec::Vec<String>,
}
pub type SignatureProperties = SignaturePropertiesType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignaturePropertiesType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "SignatureProperty")]
    pub signature_property: ::std::vec::Vec<SignaturePropertyType>,
}
pub type SignatureProperty = SignaturePropertyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignaturePropertyType {
    #[serde(rename = "@Target")]
    pub target: ::std::string::String,
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<SignaturePropertyTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum SignaturePropertyTypeContent {
    #[serde(rename = "any35")]
    Any(String),
    #[serde(rename = "$text")]
    Text(::std::string::String),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "SignedInfo")]
    pub signed_info: SignedInfoType,
    #[serde(rename = "SignatureValue")]
    pub signature_value: SignatureValueType,
    #[serde(default, rename = "KeyInfo")]
    pub key_info: ::core::option::Option<KeyInfoType>,
    #[serde(default, rename = "Object")]
    pub object: ::std::vec::Vec<ObjectType>,
}
pub type SignatureValue = SignatureValueType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureValueType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$text")]
    pub content: ::std::string::String,
}
pub type SignedInfo = SignedInfoType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignedInfoType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "CanonicalizationMethod")]
    pub canonicalization_method: CanonicalizationMethodType,
    #[serde(rename = "SignatureMethod")]
    pub signature_method: SignatureMethodType,
    #[serde(default, rename = "Reference")]
    pub reference: ::std::vec::Vec<ReferenceType>,
}
pub type Transform = TransformType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TransformType {
    #[serde(rename = "@Algorithm")]
    pub algorithm: ::std::string::String,
    #[serde(default, rename = "$value")]
    pub content: ::std::vec::Vec<TransformTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum TransformTypeContent {
    #[serde(rename = "any11")]
    Any(String),
    #[serde(rename = "XPath")]
    XPath(::std::string::String),
    #[serde(rename = "$text")]
    Text(::std::string::String),
}
pub type Transforms = TransformsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TransformsType {
    #[serde(default, rename = "Transform")]
    pub transform: ::std::vec::Vec<TransformType>,
}
pub type X509Data = X509DataType;
#[derive(Debug, Deserialize, Serialize)]
pub struct X509DataType {
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<X509DataTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum X509DataTypeContent {
    #[serde(rename = "X509IssuerSerial")]
    X509IssuerSerial(X509IssuerSerialType),
    #[serde(rename = "X509SKI")]
    X509Ski(::std::string::String),
    #[serde(rename = "X509SubjectName")]
    X509SubjectName(::std::string::String),
    #[serde(rename = "X509Certificate")]
    X509Certificate(::std::string::String),
    #[serde(rename = "X509CRL")]
    X509Crl(::std::string::String),
    #[serde(rename = "any21")]
    Any(String),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct X509IssuerSerialType {
    #[serde(rename = "X509IssuerName")]
    pub x509_issuer_name: ::std::string::String,
    #[serde(rename = "X509SerialNumber")]
    pub x509_serial_number: ::core::primitive::i32,
}
