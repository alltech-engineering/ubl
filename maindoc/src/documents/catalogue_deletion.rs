#[derive(Debug, Deserialize, Serialize)]
/// A document used to cancel an entire Catalogue.
///
/// UBL Dictionary Entry Name: `Catalogue Deletion. Details`
///
/// Generated from XSD type `CatalogueDeletionType`.
pub struct CatalogueDeletion {
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
/// Text, assigned by the sender, that identifies this document to business users.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// The effective date, assigned by the seller, on which the Catalogue expires.
    #[serde(default, rename = "EffectiveDate")]
    pub effective_date: Option<udt::DateTime>,
/// The effective time, assigned by the seller, at which the Catalogue expires.
    #[serde(default, rename = "EffectiveTime")]
    pub effective_time: Option<udt::DateTime>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies the current version of the Catalogue.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// Textual description of the document instance.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The period during which the Deletion of the catalogue becomes effective. This may be given as start
/// (after date) and end dates (before date).
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<cac::Period>,
/// A reference to the Catalogue being deleted.
    #[serde(rename = "DeletedCatalogueReference")]
    pub deleted_catalogue_reference: cac::CatalogueReference,
/// A contract or framework agreement with which the Catalogue was associated.
    #[serde(default, rename = "ReferencedContract")]
    pub referenced_contract: Vec<cac::Contract>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who receives the Catalogue Deletion.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// The Party who sends the Catalogue Deletion.
    #[serde(rename = "ProviderParty")]
    pub provider_party: cac::Party,
/// The seller.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
/// The customer party responsible for the contracts with which the Catalogue was associated.
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: Option<cac::CustomerParty>,
}
