//! UBL 2.5 common components — generated from the OASIS UBL 2.5 XSD schemas
//! (`spec/cs01-UBL-2.5/xsd/common/`) via `xsd-parser`.
//! Do not edit by hand; regenerate with `cargo run -p xsd-gen` then run the split script.

#![allow(unused_imports, dead_code, non_snake_case, clippy::all)]

use serde::{Deserialize, Serialize};

pub mod cbc;
pub mod qdt;
pub mod sac;
pub mod sbc;
pub mod xs;

// Signature-related types from UBL-CommonSignatureComponents
// (these live at the schema root level, not inside a submodule)

#[derive(Debug, Deserialize, Serialize)]
pub struct UblDocumentSignatures {
    #[serde(default, rename = "SignatureInformation")]
    pub signature_information: Vec<sac::SignatureInformation>,
}

pub type ArchiveTimeStamp = xades::GenericTimeStampType;

pub type AttributeCertificateRefsV2 = CompleteCertificateRefsTypeV2Type;

#[derive(Debug, Deserialize, Serialize)]
pub struct CompleteCertificateRefsTypeV2Type {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "CertRefs")]
    pub cert_refs: xades::CertIdListV2Type,
}

pub type CompleteCertificateRefsV2 = CompleteCertificateRefsTypeV2Type;

#[derive(Debug, Deserialize, Serialize)]
pub struct RecomputedDigestValue {
    #[serde(rename = "@Order")]
    pub order: i32,
    #[serde(rename = "$text")]
    pub content: std::string::String,
}

pub type RefsOnlyTimeStampV2 = xades::GenericTimeStampType;

#[derive(Debug, Deserialize, Serialize)]
pub struct RenewedDigests {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "DigestMethod")]
    pub digest_method: ds::DigestMethod,
    #[serde(default, rename = "RecomputedDigestValue")]
    pub recomputed_digest_value: Vec<RecomputedDigestValue>,
}

pub type SpDocSpecification = xades::ObjectIdentifier;

pub type SigAndRefsTimeStampV2 = xades::GenericTimeStampType;

#[derive(Debug, Deserialize, Serialize)]
pub struct SignaturePolicyStore {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(rename = "$value")]
    pub content: Vec<SignaturePolicyStoreTypeContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SignaturePolicyStoreTypeContent {
    #[serde(rename = "SPDocSpecification")]
    SpDocSpecification(xades::ObjectIdentifier),
    #[serde(rename = "SignaturePolicyDocument")]
    SignaturePolicyDocument(std::string::String),
    #[serde(rename = "SigPolDocLocalURI")]
    SigPolDocLocalUri(String),
}

pub type TimeStampValidationData = ValidationDataType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationDataType {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "@URI")]
    pub uri: Option<String>,
    #[serde(default, rename = "CertificateValues")]
    pub certificate_values: Option<xades::CertificateValues>,
    #[serde(default, rename = "RevocationValues")]
    pub revocation_values: Option<xades::RevocationValues>,
}
