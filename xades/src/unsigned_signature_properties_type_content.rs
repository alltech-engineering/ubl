#[derive(Debug, Deserialize, Serialize)]
pub enum UnsignedSignaturePropertiesTypeContent {
    #[serde(rename = "CounterSignature")]
    CounterSignature(CounterSignature),
    #[serde(rename = "SignatureTimeStamp")]
    SignatureTimeStamp(GenericTimeStampType),
    #[serde(rename = "CompleteCertificateRefs")]
    CompleteCertificateRefs(CompleteCertificateRefs),
    #[serde(rename = "CompleteRevocationRefs")]
    CompleteRevocationRefs(CompleteRevocationRefs),
    #[serde(rename = "AttributeCertificateRefs")]
    AttributeCertificateRefs(CompleteCertificateRefs),
    #[serde(rename = "AttributeRevocationRefs")]
    AttributeRevocationRefs(CompleteRevocationRefs),
    #[serde(rename = "SigAndRefsTimeStamp")]
    SigAndRefsTimeStamp(GenericTimeStampType),
    #[serde(rename = "RefsOnlyTimeStamp")]
    RefsOnlyTimeStamp(GenericTimeStampType),
    #[serde(rename = "CertificateValues")]
    CertificateValues(CertificateValues),
    #[serde(rename = "RevocationValues")]
    RevocationValues(RevocationValues),
    #[serde(rename = "AttrAuthoritiesCertValues")]
    AttrAuthoritiesCertValues(CertificateValues),
    #[serde(rename = "AttributeRevocationValues")]
    AttributeRevocationValues(RevocationValues),
    #[serde(rename = "ArchiveTimeStamp")]
    ArchiveTimeStamp(GenericTimeStampType),
    #[serde(rename = "any77")]
    Any(String),
}
