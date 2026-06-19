#[derive(Debug, Deserialize, Serialize)]
/// A class to define an application for a Certificate of Origin (CoO).
///
/// UBL Dictionary Entry Name: `Certificate Of Origin Application. Details`
///
/// Generated from XSD type `CertificateOfOriginApplicationType`.
pub struct CertificateOfOriginApplication {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for a reference as part of the CoO application.
    #[serde(rename = "ReferenceID")]
    pub reference_id: cct::Identifier,
/// The type of CoO being applied for (Ordinary, Re-export, Commonwealth Preferential, etc.).
    #[serde(rename = "CertificateType")]
    pub certificate_type: cct::Text,
/// A code signifying the status of the application (revision, replacement, etc.).
    #[serde(default, rename = "ApplicationStatusCode")]
    pub application_status_code: Option<cct::Code>,
/// The latest job number given to the CoO application. This is used by the system to keep track of
/// amendments to or cancellation of any earlier applications.
    #[serde(rename = "OriginalJobID")]
    pub original_job_id: cct::Identifier,
/// An identifier for the previous job used in case the application requires query or change.
    #[serde(default, rename = "PreviousJobID")]
    pub previous_job_id: Option<cct::Identifier>,
/// Remarks by the applicant for the CoO.
    #[serde(default, rename = "Remarks")]
    pub remarks: Vec<cct::Text>,
/// The shipment of goods covered by the CoO.
    #[serde(rename = "Shipment")]
    pub shipment: crate::Shipment,
/// A party providing an endorsement to the CoO.
    #[serde(default, rename = "EndorserParty")]
    pub endorser_party: Vec<crate::EndorserParty>,
/// The Party who prepares this Certificate of Origin Application. This Party is normally an individual,
/// a group or a body.
    #[serde(rename = "PreparationParty")]
    pub preparation_party: crate::Party,
/// The authorised Organisation who issues the Certificate of Origin requested by this application.
    #[serde(rename = "IssuerParty")]
    pub issuer_party: crate::Party,
/// The Party who exports the goods or has similar right of disposal over them at the time of export.
    #[serde(default, rename = "ExporterParty")]
    pub exporter_party: Option<crate::Party>,
/// The Party who imports the goods, or on whose behalf the goods are being imported.
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: Option<crate::Party>,
/// The country where the requested CoO will be issued.
    #[serde(rename = "IssuingCountry")]
    pub issuing_country: crate::Country,
/// An interested party to which the CoO is to be distributed.
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: Vec<crate::DocumentDistribution>,
/// A reference to a document supporting this application.
    #[serde(default, rename = "SupportingDocumentReference")]
    pub supporting_document_reference: Vec<crate::DocumentReference>,
/// A signature applied to this application.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<crate::Signature>,
}
