#[derive(Debug, Deserialize, Serialize)]
/// A document providing a temporary export license, also knowned as an ATA Carnet
///
/// UBL Dictionary Entry Name: `Goods Item Passport. Details`
///
/// Generated from XSD type `GoodsItemPassportType`.
pub struct GoodsItemPassport {
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
/// The reason for importing the goods, expressed as a code.
    #[serde(default, rename = "StatusCode")]
    pub status_code: Option<cct::Code>,
/// The reason for importing the goods, expressed as text in one or more languages.
    #[serde(default, rename = "Status")]
    pub status: Vec<cct::Text>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies the current version of this request for proof
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// The reason for importing the goods, expressed as a code
    #[serde(default, rename = "ExportReasonCode")]
    pub export_reason_code: Option<cct::Code>,
/// The reason for importing the goods, expressed as text in one or more languages
    #[serde(default, rename = "ExportReason")]
    pub export_reason: Vec<cct::Text>,
/// The period within which this Goods Item Passport is valid
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<cac::Period>,
/// The Party who issues this Goods Item Passport.
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<cac::Party>,
/// The Party who holds the Goods Item Passport. This Party is normally the temporary exporter of the
/// Goods.
    #[serde(rename = "HolderParty")]
    pub holder_party: cac::Party,
/// The Party who accompanies the Goods while temporarily exported.
    #[serde(default, rename = "RepresentativeParty")]
    pub representative_party: Option<cac::Party>,
/// The Party who provides a guarantee for the Goods while being temporarily exported. This Party is
/// normally a chamber of commerce.
    #[serde(default, rename = "ExportingGuarantorParty")]
    pub exporting_guarantor_party: Option<cac::Party>,
/// The Party who provides a guarantee for the Goods while being temporarily imported. This Party is
/// normally a chamber of commerce.
    #[serde(default, rename = "ImportingGuarantorParty")]
    pub importing_guarantor_party: Option<cac::Party>,
/// The Party who is competent of Customs in the exporting country.
    #[serde(default, rename = "ExportingCustomsParty")]
    pub exporting_customs_party: Option<cac::Party>,
/// The Party who is competent of Customs in the importing country.
    #[serde(default, rename = "ImportingCustomsParty")]
    pub importing_customs_party: Option<cac::Party>,
/// The reference to the shipment of the goods included under this Goods Item Passport
    #[serde(rename = "Shipment")]
    pub shipment: cac::Shipment,
/// One or more counterfoils associated with this Goods Item Passport
    #[serde(default, rename = "GoodsItemPassportCounterfoil")]
    pub goods_item_passport_counterfoil: Vec<cac::GoodsItemPassportCounterfoil>,
/// A reference to the issuer's endorsement of this Goods Item Passport
    #[serde(default, rename = "IssuerEndorsement")]
    pub issuer_endorsement: Option<cac::Endorsement>,
/// One or more references to additional documents related to this Goods Item Passport
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// One or more parties to whom this document is distributed
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: Vec<cac::DocumentDistribution>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
