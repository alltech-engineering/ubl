#[derive(Debug, Deserialize, Serialize)]
pub struct Certificate {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "CertificateTypeCode")]
    pub certificate_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CertificateType")]
    pub certificate_type: Vec<super::cct::TextType>,
    #[serde(default, rename = "CertificateReferenceID")]
    pub certificate_reference_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ApplicableCategoryCode")]
    pub applicable_category_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ApplicableCategory")]
    pub applicable_category: Option<super::cct::TextType>,
    #[serde(default, rename = "CertificateURI")]
    pub certificate_uri: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Remarks")]
    pub remarks: Vec<super::cct::TextType>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<Party>,
    #[serde(default, rename = "CertificateValidityPeriod")]
    pub certificate_validity_period: Option<Period>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<Signature>,
}
