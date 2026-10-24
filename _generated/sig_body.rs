pub type UblDocumentSignatures = UblDocumentSignaturesType;

pub struct UblDocumentSignaturesType {
    #[serde(default, rename = "SignatureInformation")]
    pub signature_information: ::std::vec::Vec<sac::SignatureInformationType>,
}

pub type ArchiveTimeStamp = xades::GenericTimeStampType;

pub type AttributeCertificateRefsV2 = CompleteCertificateRefsTypeV2Type;

pub struct CompleteCertificateRefsTypeV2Type {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "CertRefs")]
    pub cert_refs: xades::CertIdListV2Type,
}

pub type CompleteCertificateRefsV2 = CompleteCertificateRefsTypeV2Type;

pub type RecomputedDigestValue = RecomputedDigestValueType;

pub struct RecomputedDigestValueType {
    #[serde(rename = "@Order")]
    pub order: ::core::primitive::i32,
    #[serde(rename = "$text")]
    pub content: std::string::String,
}

pub type RefsOnlyTimeStampV2 = xades::GenericTimeStampType;

pub type RenewedDigests = RenewedDigestsType;

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

pub struct SignaturePolicyStoreType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<SignaturePolicyStoreTypeContent>,
}

pub enum SignaturePolicyStoreTypeContent {
    #[serde(rename = "SPDocSpecification")]
    SpDocSpecification(xades::ObjectIdentifierType),
    #[serde(rename = "SignaturePolicyDocument")]
    SignaturePolicyDocument(std::string::String),
    #[serde(rename = "SigPolDocLocalURI")]
    SigPolDocLocalUri(::std::string::String),
}

pub type TimeStampValidationData = ValidationDataType;

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

