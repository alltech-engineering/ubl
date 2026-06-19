#[derive(Debug, Deserialize, Serialize)]
/// A document that describes an import customs declaration.
///
/// UBL Dictionary Entry Name: `Transit Customs Declaration. Details`
///
/// Generated from XSD type `TransitCustomsDeclarationType`.
pub struct TransitCustomsDeclaration {
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
/// An identifier for this document.
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
/// Code specifying the type of transit customs declaration.
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::Code>,
/// Code specifying the subtype of transit customs declaration.
    #[serde(default, rename = "SubTypeCode")]
    pub sub_type_code: Option<cct::Code>,
/// Code specifying the type of transaction for this export.
    #[serde(default, rename = "NatureOfTransactionCode")]
    pub nature_of_transaction_code: Option<cct::Code>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies a version of a transit customs declaration in order to distinguish updates.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// A period, assigned by the issuer, during which the information in this declaration is effective.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<cac::Period>,
/// Customs office of exit of the goods being declared for export.
    #[serde(default, rename = "ExportCustomsExitOfficeLocation")]
    pub export_customs_exit_office_location: Option<cac::Location>,
/// Customs office of exit of the goods being declared for transit.
    #[serde(default, rename = "TransitCustomsExitOfficeLocation")]
    pub transit_customs_exit_office_location: Option<cac::Location>,
/// Customs office of exit of the goods being declared for import.
    #[serde(default, rename = "ImportCustomsExitOfficeLocation")]
    pub import_customs_exit_office_location: Option<cac::Location>,
/// A geographic area in which this declaration is relevant.
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: Option<cac::Address>,
/// The Party who makes the Transit Customs Declaration, or on whose behalf the Transit Customs
/// Declaration is made. This Party is the owner of the Goods or has similar right of disposal over them
/// at the time when the Declaration is accepted.
    #[serde(default, rename = "TransitExporterParty")]
    pub transit_exporter_party: Option<cac::Party>,
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
/// The Authority who is legally responsible for processing the Declaration.
    #[serde(rename = "CustomsParty")]
    pub customs_party: cac::Party,
/// The Party who is responsible for contact on master level.
    #[serde(default, rename = "NotifierParty")]
    pub notifier_party: Option<cac::Party>,
/// The shipment related to this trade certificate.
    #[serde(default, rename = "Shipment")]
    pub shipment: Vec<cac::Shipment>,
/// A reference to a previously sent transit customs declaration.
    #[serde(default, rename = "PreviousCustomsDeclaration")]
    pub previous_customs_declaration: Vec<cac::CustomsDeclaration>,
/// A reference to a additional documents relevant for this transit customs declaration.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
