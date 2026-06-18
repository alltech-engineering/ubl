#[derive(Debug, Deserialize, Serialize)]
pub struct WorkReport {
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
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::TextType>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "WorkQuantityTotal")]
    pub work_quantity_total: Vec<cac::WorkQuantityTotal>,
    #[serde(default, rename = "ReportedPeriod")]
    pub reported_period: Option<cac::Period>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: Option<cac::OrderReference>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: Option<cac::ProjectReference>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<cac::BillingReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierParty,
    #[serde(rename = "BuyerCustomerParty")]
    pub buyer_customer_party: cac::CustomerParty,
    #[serde(default, rename = "ApproverParty")]
    pub approver_party: Option<cac::Party>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<cac::AllowanceCharge>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
    #[serde(default, rename = "StatementMonetaryTotal")]
    pub statement_monetary_total: Option<cac::MonetaryTotal>,
    #[serde(default, rename = "WorkReportLine")]
    pub work_report_line: Vec<cac::WorkReportLine>,
}
