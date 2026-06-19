#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueRequest {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "PricingUpdateRequestIndicator")]
    pub pricing_update_request_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ItemUpdateRequestIndicator")]
    pub item_update_request_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<cac::Period>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
    #[serde(rename = "ProviderParty")]
    pub provider_party: cac::Party,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<cac::SupplierParty>,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: Option<cac::CustomerParty>,
    #[serde(default, rename = "RequestedCatalogueReference")]
    pub requested_catalogue_reference: Option<cac::CatalogueReference>,
    #[serde(default, rename = "ReferencedContract")]
    pub referenced_contract: Vec<cac::Contract>,
    #[serde(default, rename = "TradingTerms")]
    pub trading_terms: Vec<cac::TradingTerms>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: Vec<cac::Address>,
    #[serde(default, rename = "RequestedLanguage")]
    pub requested_language: Option<cac::Language>,
    #[serde(default, rename = "RequestedClassificationScheme")]
    pub requested_classification_scheme: Vec<cac::ClassificationScheme>,
    #[serde(default, rename = "CatalogueRequestLine")]
    pub catalogue_request_line: Vec<cac::CatalogueRequestLine>,
}
