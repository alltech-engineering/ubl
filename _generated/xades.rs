use serde::{Deserialize, Serialize};
pub type AllDataObjectsTimeStamp = GenericTimeStampType;
pub type Any = AnyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct AnyType {
    #[serde(rename = "@any_attribute")]
    pub any_attribute: String,
    #[serde(default, rename = "$value")]
    pub content: ::std::vec::Vec<AnyTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AnyTypeContent {
    #[serde(default, rename = "$text")]
    pub text_before: ::core::option::Option<::std::string::String>,
    #[serde(rename = "any56")]
    pub any: String,
    #[serde(default, rename = "$text")]
    pub text_after_any_56: ::core::option::Option<::std::string::String>,
}
pub type ArchiveTimeStamp = GenericTimeStampType;
pub type AttrAuthoritiesCertValues = CertificateValuesType;
pub type AttributeCertificateRefs = CompleteCertificateRefsType;
pub type AttributeRevocationRefs = CompleteRevocationRefsType;
pub type AttributeRevocationValues = RevocationValuesType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CrlIdentifierType {
    #[serde(default, rename = "@URI")]
    pub uri: ::core::option::Option<::std::string::String>,
    #[serde(rename = "Issuer")]
    pub issuer: ::std::string::String,
    #[serde(rename = "IssueTime")]
    pub issue_time: ::std::string::String,
    #[serde(default, rename = "Number")]
    pub number: ::core::option::Option<::core::primitive::i32>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CrlRefType {
    #[serde(rename = "DigestAlgAndValue")]
    pub digest_alg_and_value: DigestAlgAndValueType,
    #[serde(default, rename = "CRLIdentifier")]
    pub crl_identifier: ::core::option::Option<CrlIdentifierType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CrlRefsType {
    #[serde(default, rename = "CRLRef")]
    pub crl_ref: ::std::vec::Vec<CrlRefType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CrlValuesType {
    #[serde(default, rename = "EncapsulatedCRLValue")]
    pub encapsulated_crl_value: ::std::vec::Vec<EncapsulatedPkiDataType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CertIdListType {
    #[serde(default, rename = "Cert")]
    pub cert: ::std::vec::Vec<CertIdType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CertIdListV2Type {
    #[serde(default, rename = "Cert")]
    pub cert: ::std::vec::Vec<CertIdTypeV2Type>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CertIdType {
    #[serde(default, rename = "@URI")]
    pub uri: ::core::option::Option<::std::string::String>,
    #[serde(rename = "CertDigest")]
    pub cert_digest: DigestAlgAndValueType,
    #[serde(rename = "IssuerSerial")]
    pub issuer_serial: super::ds::ubl_xmldsig_core_schema_25::X509IssuerSerialType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CertIdTypeV2Type {
    #[serde(default, rename = "@URI")]
    pub uri: ::core::option::Option<::std::string::String>,
    #[serde(rename = "CertDigest")]
    pub cert_digest: DigestAlgAndValueType,
    #[serde(default, rename = "IssuerSerialV2")]
    pub issuer_serial_v2: ::core::option::Option<::std::string::String>,
}
pub type CertificateValues = CertificateValuesType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CertificateValuesType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "$value")]
    pub content: ::std::vec::Vec<CertificateValuesTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum CertificateValuesTypeContent {
    #[serde(rename = "EncapsulatedX509Certificate")]
    EncapsulatedX509Certificate(EncapsulatedPkiDataType),
    #[serde(rename = "OtherCertificate")]
    OtherCertificate(AnyType),
}
#[derive(Debug, Deserialize, Serialize)]
pub enum CertifiedRoleTypeV2Type {
    #[serde(rename = "X509AttributeCertificate")]
    X509AttributeCertificate(EncapsulatedPkiDataType),
    #[serde(rename = "OtherAttributeCertificate")]
    OtherAttributeCertificate(AnyType),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CertifiedRolesListType {
    #[serde(default, rename = "CertifiedRole")]
    pub certified_role: ::std::vec::Vec<EncapsulatedPkiDataType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CertifiedRolesListTypeV2Type {
    #[serde(default, rename = "CertifiedRole")]
    pub certified_role: ::std::vec::Vec<CertifiedRoleTypeV2Type>,
}
pub type CertifiedRolesV2 = CertifiedRolesListTypeV2Type;
pub type ClaimedRoles = ClaimedRolesListType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimedRolesListType {
    #[serde(default, rename = "ClaimedRole")]
    pub claimed_role: ::std::vec::Vec<AnyType>,
}
pub type CommitmentTypeIndication = CommitmentTypeIndicationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CommitmentTypeIndicationType {
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<CommitmentTypeIndicationTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum CommitmentTypeIndicationTypeContent {
    #[serde(rename = "CommitmentTypeId")]
    CommitmentTypeId(ObjectIdentifierType),
    #[serde(rename = "ObjectReference")]
    ObjectReference(::std::string::String),
    #[serde(rename = "AllSignedDataObjects")]
    AllSignedDataObjects(::std::string::String),
    #[serde(rename = "CommitmentTypeQualifiers")]
    CommitmentTypeQualifiers(CommitmentTypeQualifiersListType),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CommitmentTypeQualifiersListType {
    #[serde(default, rename = "CommitmentTypeQualifier")]
    pub commitment_type_qualifier: ::std::vec::Vec<AnyType>,
}
pub type CompleteCertificateRefs = CompleteCertificateRefsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CompleteCertificateRefsType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "CertRefs")]
    pub cert_refs: CertIdListType,
}
pub type CompleteRevocationRefs = CompleteRevocationRefsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CompleteRevocationRefsType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "CRLRefs")]
    pub crl_refs: ::core::option::Option<CrlRefsType>,
    #[serde(default, rename = "OCSPRefs")]
    pub ocsp_refs: ::core::option::Option<OcspRefsType>,
    #[serde(default, rename = "OtherRefs")]
    pub other_refs: ::core::option::Option<OtherCertStatusRefsType>,
}
pub type CounterSignature = CounterSignatureType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CounterSignatureType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "Signature")]
    pub signature: super::ds::ubl_xmldsig_core_schema_25::SignatureType,
}
pub type DataObjectFormat = DataObjectFormatType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DataObjectFormatType {
    #[serde(rename = "@ObjectReference")]
    pub object_reference: ::std::string::String,
    #[serde(default, rename = "Description")]
    pub description: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "ObjectIdentifier")]
    pub object_identifier: ::core::option::Option<ObjectIdentifierType>,
    #[serde(default, rename = "MimeType")]
    pub mime_type: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "Encoding")]
    pub encoding: ::core::option::Option<::std::string::String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DigestAlgAndValueType {
    #[serde(rename = "DigestMethod")]
    pub digest_method: super::ds::ubl_xmldsig_core_schema_25::DigestMethodType,
    #[serde(rename = "DigestValue")]
    pub digest_value: ::std::string::String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentationReferencesType {
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<DocumentationReferencesTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentationReferencesTypeContent {
    #[serde(rename = "DocumentationReference")]
    pub documentation_reference: ::std::string::String,
}
pub type EncapsulatedPkiData = EncapsulatedPkiDataType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EncapsulatedPkiDataType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@Encoding")]
    pub encoding: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$text")]
    pub content: ::std::string::String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GenericTimeStampType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<GenericTimeStampTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum GenericTimeStampTypeContent {
    #[serde(rename = "Include")]
    Include(IncludeType),
    #[serde(rename = "ReferenceInfo")]
    ReferenceInfo(ReferenceInfoType),
    #[serde(rename = "CanonicalizationMethod")]
    CanonicalizationMethod(super::ds::ubl_xmldsig_core_schema_25::CanonicalizationMethodType),
    #[serde(rename = "EncapsulatedTimeStamp")]
    EncapsulatedTimeStamp(EncapsulatedPkiDataType),
    #[serde(rename = "XMLTimeStamp")]
    XmlTimeStamp(AnyType),
}
#[derive(Debug, Deserialize, Serialize)]
pub struct IdentifierType {
    #[serde(default, rename = "@Qualifier")]
    pub qualifier: ::core::option::Option<QualifierType>,
    #[serde(default, rename = "$text")]
    pub content: ::std::string::String,
}
pub type Include = IncludeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct IncludeType {
    #[serde(rename = "@URI")]
    pub uri: ::std::string::String,
    #[serde(default, rename = "@referencedData")]
    pub referenced_data: ::core::option::Option<::core::primitive::bool>,
}
pub type IndividualDataObjectsTimeStamp = GenericTimeStampType;
#[derive(Debug, Deserialize, Serialize)]
pub struct IntegerListType {
    #[serde(default, rename = "int")]
    pub int: ::std::vec::Vec<::core::primitive::i32>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct NoticeReferenceType {
    #[serde(rename = "Organization")]
    pub organization: ::std::string::String,
    #[serde(rename = "NoticeNumbers")]
    pub notice_numbers: IntegerListType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OcspIdentifierType {
    #[serde(default, rename = "@URI")]
    pub uri: ::core::option::Option<::std::string::String>,
    #[serde(rename = "ResponderID")]
    pub responder_id: ResponderIdType,
    #[serde(rename = "ProducedAt")]
    pub produced_at: ::std::string::String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OcspRefType {
    #[serde(rename = "OCSPIdentifier")]
    pub ocsp_identifier: OcspIdentifierType,
    #[serde(default, rename = "DigestAlgAndValue")]
    pub digest_alg_and_value: ::core::option::Option<DigestAlgAndValueType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OcspRefsType {
    #[serde(default, rename = "OCSPRef")]
    pub ocsp_ref: ::std::vec::Vec<OcspRefType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OcspValuesType {
    #[serde(default, rename = "EncapsulatedOCSPValue")]
    pub encapsulated_ocsp_value: ::std::vec::Vec<EncapsulatedPkiDataType>,
}
pub type ObjectIdentifier = ObjectIdentifierType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectIdentifierType {
    #[serde(rename = "Identifier")]
    pub identifier: IdentifierType,
    #[serde(default, rename = "Description")]
    pub description: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "DocumentationReferences")]
    pub documentation_references: ::core::option::Option<DocumentationReferencesType>,
}
pub type OtherAttributeCertificate = AnyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct OtherCertStatusRefsType {
    #[serde(default, rename = "OtherRef")]
    pub other_ref: ::std::vec::Vec<AnyType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OtherCertStatusValuesType {
    #[serde(default, rename = "OtherValue")]
    pub other_value: ::std::vec::Vec<AnyType>,
}
pub type OtherTimeStamp = GenericTimeStampType;
pub type OtherTimeStampType = GenericTimeStampType;
#[derive(Debug, Deserialize, Serialize)]
pub enum QualifierType {
    #[serde(rename = "OIDAsURI")]
    OidAsUri,
    #[serde(rename = "OIDAsURN")]
    OidAsUrn,
}
pub type QualifyingProperties = QualifyingPropertiesType;
pub type QualifyingPropertiesReference = QualifyingPropertiesReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct QualifyingPropertiesReferenceType {
    #[serde(rename = "@URI")]
    pub uri: ::std::string::String,
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct QualifyingPropertiesType {
    #[serde(rename = "@Target")]
    pub target: ::std::string::String,
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "SignedProperties")]
    pub signed_properties: ::core::option::Option<SignedPropertiesType>,
    #[serde(default, rename = "UnsignedProperties")]
    pub unsigned_properties: ::core::option::Option<UnsignedPropertiesType>,
}
pub type ReferenceInfo = ReferenceInfoType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ReferenceInfoType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "@URI")]
    pub uri: ::core::option::Option<::std::string::String>,
    #[serde(rename = "DigestMethod")]
    pub digest_method: super::ds::ubl_xmldsig_core_schema_25::DigestMethodType,
    #[serde(rename = "DigestValue")]
    pub digest_value: ::std::string::String,
}
pub type RefsOnlyTimeStamp = GenericTimeStampType;
#[derive(Debug, Deserialize, Serialize)]
pub enum ResponderIdType {
    #[serde(rename = "ByName")]
    ByName(::std::string::String),
    #[serde(rename = "ByKey")]
    ByKey(::std::string::String),
}
pub type RevocationValues = RevocationValuesType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RevocationValuesType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "CRLValues")]
    pub crl_values: ::core::option::Option<CrlValuesType>,
    #[serde(default, rename = "OCSPValues")]
    pub ocsp_values: ::core::option::Option<OcspValuesType>,
    #[serde(default, rename = "OtherValues")]
    pub other_values: ::core::option::Option<OtherCertStatusValuesType>,
}
pub type Spuri = ::std::string::String;
pub type SpUserNotice = SpUserNoticeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SpUserNoticeType {
    #[serde(default, rename = "NoticeRef")]
    pub notice_ref: ::core::option::Option<NoticeReferenceType>,
    #[serde(default, rename = "ExplicitText")]
    pub explicit_text: ::core::option::Option<::std::string::String>,
}
pub type SigAndRefsTimeStamp = GenericTimeStampType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SigPolicyQualifiersListType {
    #[serde(default, rename = "SigPolicyQualifier")]
    pub sig_policy_qualifier: ::std::vec::Vec<AnyType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SignaturePolicyIdType {
    #[serde(rename = "SigPolicyId")]
    pub sig_policy_id: ObjectIdentifierType,
    #[serde(default, rename = "Transforms")]
    pub transforms: ::core::option::Option<super::ds::ubl_xmldsig_core_schema_25::TransformsType>,
    #[serde(rename = "SigPolicyHash")]
    pub sig_policy_hash: DigestAlgAndValueType,
    #[serde(default, rename = "SigPolicyQualifiers")]
    pub sig_policy_qualifiers: ::core::option::Option<SigPolicyQualifiersListType>,
}
pub type SignaturePolicyIdentifier = SignaturePolicyIdentifierType;
#[derive(Debug, Deserialize, Serialize)]
pub enum SignaturePolicyIdentifierType {
    #[serde(rename = "SignaturePolicyId")]
    SignaturePolicyId(SignaturePolicyIdType),
    #[serde(rename = "SignaturePolicyImplied")]
    SignaturePolicyImplied(::std::string::String),
}
pub type SignatureProductionPlace = SignatureProductionPlaceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureProductionPlaceType {
    #[serde(default, rename = "City")]
    pub city: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "StateOrProvince")]
    pub state_or_province: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "PostalCode")]
    pub postal_code: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "CountryName")]
    pub country_name: ::core::option::Option<::std::string::String>,
}
pub type SignatureProductionPlaceV2 = SignatureProductionPlaceV2Type;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureProductionPlaceV2Type {
    #[serde(default, rename = "City")]
    pub city: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "StreetAddress")]
    pub street_address: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "StateOrProvince")]
    pub state_or_province: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "PostalCode")]
    pub postal_code: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "CountryName")]
    pub country_name: ::core::option::Option<::std::string::String>,
}
pub type SignatureTimeStamp = GenericTimeStampType;
pub type SignedAssertion = AnyType;
pub type SignedAssertions = SignedAssertionsListType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignedAssertionsListType {
    #[serde(default, rename = "SignedAssertion")]
    pub signed_assertion: ::std::vec::Vec<AnyType>,
}
pub type SignedDataObjectProperties = SignedDataObjectPropertiesType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignedDataObjectPropertiesType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "DataObjectFormat")]
    pub data_object_format: ::std::vec::Vec<DataObjectFormatType>,
    #[serde(default, rename = "CommitmentTypeIndication")]
    pub commitment_type_indication: ::std::vec::Vec<CommitmentTypeIndicationType>,
    #[serde(default, rename = "AllDataObjectsTimeStamp")]
    pub all_data_objects_time_stamp: ::std::vec::Vec<GenericTimeStampType>,
    #[serde(default, rename = "IndividualDataObjectsTimeStamp")]
    pub individual_data_objects_time_stamp: ::std::vec::Vec<GenericTimeStampType>,
    #[serde(default, rename = "any75")]
    pub any: ::std::vec::Vec<String>,
}
pub type SignedProperties = SignedPropertiesType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignedPropertiesType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "SignedSignatureProperties")]
    pub signed_signature_properties: ::core::option::Option<SignedSignaturePropertiesType>,
    #[serde(default, rename = "SignedDataObjectProperties")]
    pub signed_data_object_properties: ::core::option::Option<SignedDataObjectPropertiesType>,
}
pub type SignedSignatureProperties = SignedSignaturePropertiesType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignedSignaturePropertiesType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "SigningTime")]
    pub signing_time: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "SigningCertificate")]
    pub signing_certificate: ::core::option::Option<CertIdListType>,
    #[serde(default, rename = "SigningCertificateV2")]
    pub signing_certificate_v2: ::core::option::Option<CertIdListV2Type>,
    #[serde(default, rename = "SignaturePolicyIdentifier")]
    pub signature_policy_identifier: ::core::option::Option<SignaturePolicyIdentifierType>,
    #[serde(default, rename = "SignatureProductionPlace")]
    pub signature_production_place: ::core::option::Option<SignatureProductionPlaceType>,
    #[serde(default, rename = "SignatureProductionPlaceV2")]
    pub signature_production_place_v2: ::core::option::Option<SignatureProductionPlaceV2Type>,
    #[serde(default, rename = "SignerRole")]
    pub signer_role: ::core::option::Option<SignerRoleType>,
    #[serde(default, rename = "SignerRoleV2")]
    pub signer_role_v2: ::core::option::Option<SignerRoleV2Type>,
    #[serde(default, rename = "any73")]
    pub any: ::std::vec::Vec<String>,
}
pub type SignerRole = SignerRoleType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignerRoleType {
    #[serde(default, rename = "ClaimedRoles")]
    pub claimed_roles: ::core::option::Option<ClaimedRolesListType>,
    #[serde(default, rename = "CertifiedRoles")]
    pub certified_roles: ::core::option::Option<CertifiedRolesListType>,
}
pub type SignerRoleV2 = SignerRoleV2Type;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignerRoleV2Type {
    #[serde(default, rename = "ClaimedRoles")]
    pub claimed_roles: ::core::option::Option<ClaimedRolesListType>,
    #[serde(default, rename = "CertifiedRolesV2")]
    pub certified_roles_v2: ::core::option::Option<CertifiedRolesListTypeV2Type>,
    #[serde(default, rename = "SignedAssertions")]
    pub signed_assertions: ::core::option::Option<SignedAssertionsListType>,
}
pub type SigningCertificate = CertIdListType;
pub type SigningCertificateV2 = CertIdListV2Type;
pub type SigningTime = ::std::string::String;
pub type UnsignedDataObjectProperties = UnsignedDataObjectPropertiesType;
#[derive(Debug, Deserialize, Serialize)]
pub struct UnsignedDataObjectPropertiesType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "UnsignedDataObjectProperty")]
    pub unsigned_data_object_property: ::std::vec::Vec<AnyType>,
}
pub type UnsignedProperties = UnsignedPropertiesType;
#[derive(Debug, Deserialize, Serialize)]
pub struct UnsignedPropertiesType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(default, rename = "UnsignedSignatureProperties")]
    pub unsigned_signature_properties: ::core::option::Option<UnsignedSignaturePropertiesType>,
    #[serde(default, rename = "UnsignedDataObjectProperties")]
    pub unsigned_data_object_properties: ::core::option::Option<UnsignedDataObjectPropertiesType>,
}
pub type UnsignedSignatureProperties = UnsignedSignaturePropertiesType;
#[derive(Debug, Deserialize, Serialize)]
pub struct UnsignedSignaturePropertiesType {
    #[serde(default, rename = "@Id")]
    pub id: ::core::option::Option<::std::string::String>,
    #[serde(rename = "$value")]
    pub content: ::std::vec::Vec<UnsignedSignaturePropertiesTypeContent>,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum UnsignedSignaturePropertiesTypeContent {
    #[serde(rename = "CounterSignature")]
    CounterSignature(CounterSignatureType),
    #[serde(rename = "SignatureTimeStamp")]
    SignatureTimeStamp(GenericTimeStampType),
    #[serde(rename = "CompleteCertificateRefs")]
    CompleteCertificateRefs(CompleteCertificateRefsType),
    #[serde(rename = "CompleteRevocationRefs")]
    CompleteRevocationRefs(CompleteRevocationRefsType),
    #[serde(rename = "AttributeCertificateRefs")]
    AttributeCertificateRefs(CompleteCertificateRefsType),
    #[serde(rename = "AttributeRevocationRefs")]
    AttributeRevocationRefs(CompleteRevocationRefsType),
    #[serde(rename = "SigAndRefsTimeStamp")]
    SigAndRefsTimeStamp(GenericTimeStampType),
    #[serde(rename = "RefsOnlyTimeStamp")]
    RefsOnlyTimeStamp(GenericTimeStampType),
    #[serde(rename = "CertificateValues")]
    CertificateValues(CertificateValuesType),
    #[serde(rename = "RevocationValues")]
    RevocationValues(RevocationValuesType),
    #[serde(rename = "AttrAuthoritiesCertValues")]
    AttrAuthoritiesCertValues(CertificateValuesType),
    #[serde(rename = "AttributeRevocationValues")]
    AttributeRevocationValues(RevocationValuesType),
    #[serde(rename = "ArchiveTimeStamp")]
    ArchiveTimeStamp(GenericTimeStampType),
    #[serde(rename = "any77")]
    Any(String),
}
pub type X509AttributeCertificate = EncapsulatedPkiDataType;
pub type XAdEsTimeStamp = GenericTimeStampType;
pub type XAdEsTimeStampType = GenericTimeStampType;
