#[derive(Debug, Deserialize, Serialize)]
pub struct CertificateOfOriginApplication {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "ReferenceID")]
    pub reference_id: cct::Identifier,
    #[serde(rename = "CertificateType")]
    pub certificate_type: cct::Text,
    #[serde(default, rename = "ApplicationStatusCode")]
    pub application_status_code: Option<cct::Code>,
    #[serde(rename = "OriginalJobID")]
    pub original_job_id: cct::Identifier,
    #[serde(default, rename = "PreviousJobID")]
    pub previous_job_id: Option<cct::Identifier>,
    #[serde(default, rename = "Remarks")]
    pub remarks: Vec<cct::Text>,
    #[serde(rename = "Shipment")]
    pub shipment: crate::Shipment,
    #[serde(default, rename = "EndorserParty")]
    pub endorser_party: Vec<crate::EndorserParty>,
    #[serde(rename = "PreparationParty")]
    pub preparation_party: crate::Party,
    #[serde(rename = "IssuerParty")]
    pub issuer_party: crate::Party,
    #[serde(default, rename = "ExporterParty")]
    pub exporter_party: Option<crate::Party>,
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: Option<crate::Party>,
    #[serde(rename = "IssuingCountry")]
    pub issuing_country: crate::Country,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: Vec<crate::DocumentDistribution>,
    #[serde(default, rename = "SupportingDocumentReference")]
    pub supporting_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<crate::Signature>,
}
