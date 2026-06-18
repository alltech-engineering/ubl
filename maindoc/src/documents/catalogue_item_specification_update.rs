#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueItemSpecificationUpdate {
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
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<cac::Period>,
    #[serde(rename = "RelatedCatalogueReference")]
    pub related_catalogue_reference: cac::CatalogueReference,
    #[serde(default, rename = "ReferencedContract")]
    pub referenced_contract: Vec<cac::Contract>,
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
    #[serde(default, rename = "DefaultLanguage")]
    pub default_language: Option<cac::Language>,
    #[serde(default, rename = "CatalogueItemSpecificationUpdateLine")]
    pub catalogue_item_specification_update_line:
        Vec<cac::CatalogueItemSpecificationUpdateLine>,
}
