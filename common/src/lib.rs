//! UBL 2.5 common components — generated from the OASIS UBL 2.5 XSD schemas
//! (`spec/cs01-UBL-2.5/xsd/common/`) via `xsd-parser`.
//! Do not edit by hand; regenerate with `cargo run -p xsd-gen` then run the split script.

#![allow(unused_imports, dead_code, non_snake_case, clippy::all)]

use serde::{Deserialize, Serialize};

pub mod cac;
pub mod cbc;
pub mod cct;
pub mod ds;
pub mod dsig_11;
pub mod ext;
pub mod qdt;
pub mod sac;
pub mod sbc;
pub mod udt;
pub mod xades;
pub mod xs;

// Signature-related types from UBL-CommonSignatureComponents
// (these live at the schema root level, not inside a submodule)
pub type UblDocumentSignatures = UblDocumentSignaturesType;

#[derive(Debug, Deserialize, Serialize)]
pub struct UblDocumentSignaturesType {
    #[serde(default, rename = "SignatureInformation")]
    pub signature_information: ::std::vec::Vec<sac::SignatureInformationType>,
}

pub type ArchiveTimeStamp = xades::GenericTimeStampType;

pub type AttributeCertificateRefsV2 = CompleteCertificateRefsTypeV2Type;

#[derive(Debug, Deserialize, Serialize)]
pub struct CompleteCertificateRefsTypeV2Type {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "CertRefs")]
    pub cert_refs: xades::CertIdListV2Type,
}

pub type CompleteCertificateRefsV2 = CompleteCertificateRefsTypeV2Type;

pub type RecomputedDigestValue = RecomputedDigestValueType;

#[derive(Debug, Deserialize, Serialize)]
pub struct RecomputedDigestValueType {
    #[serde(rename = "@Order")]
    pub order: ::core::primitive::i32,
    #[serde(rename = "$text")]
    pub content: std::string::String,
}

pub type RefsOnlyTimeStampV2 = xades::GenericTimeStampType;

pub type RenewedDigests = RenewedDigestsType;

#[derive(Debug, Deserialize, Serialize)]
pub struct RenewedDigestsType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "DigestMethod")]
    pub digest_method: ds::ubl_xmldsig_core_schema_25::DigestMethodType,
    #[serde(default, rename = "RecomputedDigestValue")]
    pub recomputed_digest_value: ::std::vec::Vec<RecomputedDigestValueType>,
}

pub type SpDocSpecification = xades::ObjectIdentifierType;

pub type SigAndRefsTimeStampV2 = xades::GenericTimeStampType;

pub type SignaturePolicyStore = SignaturePolicyStoreType;

#[derive(Debug, Deserialize, Serialize)]
pub struct SignaturePolicyStoreType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<SignaturePolicyStoreTypeContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SignaturePolicyStoreTypeContent {
    #[serde(rename = "SPDocSpecification")]
    SpDocSpecification(xades::ObjectIdentifierType),
    #[serde(rename = "SignaturePolicyDocument")]
    SignaturePolicyDocument(std::string::String),
    #[serde(rename = "SigPolDocLocalURI")]
    SigPolDocLocalUri(::std::string::String),
}

pub type TimeStampValidationData = ValidationDataType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationDataType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@URI")]
    pub uri: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "CertificateValues")]
    pub certificate_values: ::core::option::Option<xades::CertificateValuesType>,
    #[serde(default, rename = "RevocationValues")]
    pub revocation_values: ::core::option::Option<xades::RevocationValuesType>,
}

