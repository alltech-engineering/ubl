#[derive(Debug, Deserialize, Serialize)]
/// A document that describes a certificate of goods for importation and exportation
///
/// UBL Dictionary Entry Name: `Goods Certificate. Details`
///
/// Generated from XSD type `GoodsCertificateType`.
pub struct GoodsCertificate {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
/// Identifies the earliest version of the UBL 2 schema for this document type that defines all of the
/// elements that might be encountered in the current instance.
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
/// Identifies a user-defined customization of UBL for a specific use.
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
/// Identifies a user-defined profile of the customization of UBL being used.
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
/// Identifies an instance of executing a profile, to associate all transactions in a collaboration.
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
/// An identifier for this document, assigned by the sender.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// A code specifying the type of goods certificate
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::Code>,
/// Textual description of this goods certificate
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies the current version of this goods certificate
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// The period of time for which this goods certificate is considered valid
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<cac::Period>,
/// A geographic area where this goods certificate is valid
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: Option<cac::Address>,
/// The Party who exports the goods or has similar right of disposal over them at the time of export.
    #[serde(default, rename = "ExporterParty")]
    pub exporter_party: Option<cac::Party>,
/// The Party who imports the goods, or on whose behalf the goods are being imported.
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: Option<cac::Party>,
/// The Party who is responsible for storing the Goods.
    #[serde(default, rename = "WarehouseParty")]
    pub warehouse_party: Option<cac::Party>,
/// The Party who is reponsible for sending the goods.
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: Option<cac::Party>,
/// The Party who receives the goods.
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: Option<cac::Party>,
/// The Party who combines individual smaller consignments into a single larger shipment (a so-called
/// consolidated consignment or shipment) which is sent to a counterpart who mirrors the consolidator's
/// activity by dividing the consolidated consignment into its original components.
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<cac::Party>,
/// The Party who issues this Goods Certificate.
    #[serde(rename = "IssuerParty")]
    pub issuer_party: cac::Party,
/// The legal Authority, when different from the issuer, who sanctions this Goods Certificate.
    #[serde(default, rename = "LegalAuthorityParty")]
    pub legal_authority_party: Option<cac::Party>,
/// The Party who applies for this Goods Certificate.
    #[serde(default, rename = "ApplicantParty")]
    pub applicant_party: Option<cac::Party>,
/// The shipment for which this goods certificate is issued
    #[serde(rename = "Shipment")]
    pub shipment: cac::Shipment,
/// Any attestations made for the goods related to this goods certificate
    #[serde(default, rename = "Attestation")]
    pub attestation: Vec<cac::Attestation>,
/// Any processing that the goods have been undergoing
    #[serde(default, rename = "GoodsProcessing")]
    pub goods_processing: Vec<cac::GoodsProcessing>,
/// A reference to the original version of the goods certificate
    #[serde(default, rename = "OriginalDocumentReference")]
    pub original_document_reference: Option<cac::DocumentReference>,
/// A reference to the previous version of the goods certificate
    #[serde(default, rename = "PreviousDocumentReference")]
    pub previous_document_reference: Option<cac::DocumentReference>,
/// A reference to an additional document associated with this goods certificate
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
