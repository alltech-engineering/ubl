#[derive(Debug, Deserialize, Serialize)]
pub struct CertificateOfOriginApplication {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ReferenceID")]
    pub reference_id: super::cct::IdentifierType,
    #[serde(rename = "CertificateType")]
    pub certificate_type: super::cct::TextType,
    #[serde(default, rename = "ApplicationStatusCode")]
    pub application_status_code: Option<super::cct::CodeType>,
    #[serde(rename = "OriginalJobID")]
    pub original_job_id: super::cct::IdentifierType,
    #[serde(default, rename = "PreviousJobID")]
    pub previous_job_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Remarks")]
    pub remarks: Vec<super::cct::TextType>,
    #[serde(rename = "Shipment")]
    pub shipment: Shipment,
    #[serde(default, rename = "EndorserParty")]
    pub endorser_party: Vec<EndorserParty>,
    #[serde(rename = "PreparationParty")]
    pub preparation_party: Party,
    #[serde(rename = "IssuerParty")]
    pub issuer_party: Party,
    #[serde(default, rename = "ExporterParty")]
    pub exporter_party: Option<Party>,
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: Option<Party>,
    #[serde(rename = "IssuingCountry")]
    pub issuing_country: Country,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: Vec<DocumentDistribution>,
    #[serde(default, rename = "SupportingDocumentReference")]
    pub supporting_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<Signature>,
}
