#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueDeletion {
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
    #[serde(default, rename = "EffectiveDate")]
    pub effective_date: Option<udt::DateTime>,
    #[serde(default, rename = "EffectiveTime")]
    pub effective_time: Option<udt::DateTime>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<cac::Period>,
    #[serde(rename = "DeletedCatalogueReference")]
    pub deleted_catalogue_reference: cac::CatalogueReference,
    #[serde(default, rename = "ReferencedContract")]
    pub referenced_contract: Vec<cac::Contract>,
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
}
