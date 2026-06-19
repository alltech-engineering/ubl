#[derive(Debug, Deserialize, Serialize)]
/// A document that describes items, prices, and price validity.
///
/// UBL Dictionary Entry Name: `Catalogue. Details`
///
/// Generated from XSD type `CatalogueType`.
pub struct Catalogue {
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
/// A code signifying whether the transaction is a replacement or an update.
    #[serde(default, rename = "ActionCode")]
    pub action_code: Option<cct::Code>,
/// Text, assigned by the sender, that identifies this document to business users.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// The date, assigned by the seller party, on which the information in the Catalogue was last revised.
    #[serde(default, rename = "RevisionDate")]
    pub revision_date: Option<udt::DateTime>,
/// The time, assigned by the Seller party, at which the information in the Catalogue was last revised.
    #[serde(default, rename = "RevisionTime")]
    pub revision_time: Option<udt::DateTime>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Textual description of the document instance.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// An identifier for the current version of the Catalogue.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// An identifier for the previous version of the Catalogue that is superseded by this version.
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<cct::Identifier>,
/// The number of Catalogue Lines in the document.
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
/// A period, assigned by the seller, during which the information in the Catalogue is effective. This
/// may be given as start and end dates or as a duration.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<cac::Period>,
/// A contract or framework agreement with which this Catalogue is associated.
    #[serde(default, rename = "ReferencedContract")]
    pub referenced_contract: Vec<cac::Contract>,
/// A reference to the source catalogue.
    #[serde(default, rename = "SourceCatalogueReference")]
    pub source_catalogue_reference: Option<cac::CatalogueReference>,
/// A reference to another document associated with this document.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
/// A geographic or territorial area to which the Catalogue as a whole applies.
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: Vec<cac::Address>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who provides this Catalogue.
    #[serde(rename = "ProviderParty")]
    pub provider_party: cac::Party,
/// The Party who receives this Catalogue.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// The seller.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
/// The customer party responsible for the contracts with which the Catalogue is associated.
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: Option<cac::CustomerParty>,
/// The trading terms associated with this Catalogue.
    #[serde(default, rename = "TradingTerms")]
    pub trading_terms: Vec<cac::TradingTerms>,
/// A line in a Catalogue describing an item of sale.
    #[serde(default, rename = "CatalogueLine")]
    pub catalogue_line: Vec<cac::CatalogueLine>,
}
