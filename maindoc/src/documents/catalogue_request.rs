#[derive(Debug, Deserialize, Serialize)]
/// A document used to request a Catalogue.
///
/// UBL Dictionary Entry Name: `Catalogue Request. Details`
///
/// Generated from XSD type `CatalogueRequestType`.
pub struct CatalogueRequest {
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
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Textual description of the document instance.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Indicates a request for a pricing update.
    #[serde(default, rename = "PricingUpdateRequestIndicator")]
    pub pricing_update_request_indicator: Option<udt::Indicator>,
/// Indicates a request for an update of the item specifications.
    #[serde(default, rename = "ItemUpdateRequestIndicator")]
    pub item_update_request_indicator: Option<udt::Indicator>,
/// The number of Catalogue Lines in this document.
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
/// The period, assigned by the Catalogue Managing party, during which the information in the Catalogue
/// requested is to be effective. This may be given as start and end dates or a duration.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<cac::Period>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who receives the Catalogue Request.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// The Party who sends the Catalogue Request.
    #[serde(rename = "ProviderParty")]
    pub provider_party: cac::Party,
/// The seller.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
/// The customer party responsible for the contracts with which the Catalogue is associated.
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: Option<cac::CustomerParty>,
/// A reference to a specific Catalogue; used if the Catalogue Request is for an update.
    #[serde(default, rename = "RequestedCatalogueReference")]
    pub requested_catalogue_reference: Option<cac::CatalogueReference>,
/// A contract or framework agreement with which the Catalogue being requested is associated.
    #[serde(default, rename = "ReferencedContract")]
    pub referenced_contract: Vec<cac::Contract>,
/// The trading terms associated with the requested Catalogue.
    #[serde(default, rename = "TradingTerms")]
    pub trading_terms: Vec<cac::TradingTerms>,
/// A reference to another document associated with this document.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
/// A reference to a territory (region, country, city, etc.) to which the requested Catalogue will
/// apply, expressed as an Address.
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: Vec<cac::Address>,
/// The language in which the Catalogue is requested to be provided.
    #[serde(default, rename = "RequestedLanguage")]
    pub requested_language: Option<cac::Language>,
/// A requested classification scheme for the requested Catalogue.
    #[serde(default, rename = "RequestedClassificationScheme")]
    pub requested_classification_scheme: Vec<cac::ClassificationScheme>,
/// An association to specific Catalogue Lines for the catalogue requested.
    #[serde(default, rename = "CatalogueRequestLine")]
    pub catalogue_request_line: Vec<cac::CatalogueRequestLine>,
}
