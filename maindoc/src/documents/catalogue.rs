#[derive(Debug, Deserialize, Serialize)]
pub struct Catalogue {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(default, rename = "ActionCode")]
    pub action_code: Option<cct::CodeType>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::TextType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "RevisionDate")]
    pub revision_date: Option<udt::DateTimeType>,
    #[serde(default, rename = "RevisionTime")]
    pub revision_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<cac::Period>,
    #[serde(default, rename = "ReferencedContract")]
    pub referenced_contract: Vec<cac::Contract>,
    #[serde(default, rename = "SourceCatalogueReference")]
    pub source_catalogue_reference: Option<cac::CatalogueReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: Vec<cac::Address>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "ProviderParty")]
    pub provider_party: cac::Party,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "TradingTerms")]
    pub trading_terms: Vec<cac::TradingTerms>,
    #[serde(default, rename = "CatalogueLine")]
    pub catalogue_line: Vec<cac::CatalogueLine>,
}
