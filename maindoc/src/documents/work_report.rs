#[derive(Debug, Deserialize, Serialize)]
/// A document used to report work performed.
///
/// UBL Dictionary Entry Name: `Work Report. Details`
///
/// Generated from XSD type `WorkReportType`.
pub struct WorkReport {
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
/// An identifier for this Work Report.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// Identifies a version of this work report.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// The date on which this work report was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// An accounting cost code applied to this Work Report.
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
/// An accounting cost centre or account to which this Work Report is charged.
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// A total quantity of work reported in this Work Report.
    #[serde(default, rename = "WorkQuantityTotal")]
    pub work_quantity_total: Vec<cac::WorkQuantityTotal>,
/// The period during which the reported work was performed.
    #[serde(default, rename = "ReportedPeriod")]
    pub reported_period: Option<cac::Period>,
/// A reference to the related Order.
    #[serde(default, rename = "OrderReference")]
    pub order_reference: Option<cac::OrderReference>,
/// A reference to the related project.
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: Option<cac::ProjectReference>,
/// A reference to a related billing document.
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<cac::BillingReference>,
/// A reference to an additional supporting document.
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
/// The party providing the work.
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierParty,
/// The party receiving the work.
    #[serde(rename = "BuyerCustomerParty")]
    pub buyer_customer_party: cac::CustomerParty,
/// The party approving this work report.
    #[serde(default, rename = "ApproverParty")]
    pub approver_party: Option<cac::Party>,
/// A discount or charge applied at the document level.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<cac::AllowanceCharge>,
/// A total amount of taxes of a particular kind applicable to this Work Report.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
/// The total amounts for this Work Report.
    #[serde(default, rename = "StatementMonetaryTotal")]
    pub statement_monetary_total: Option<cac::MonetaryTotal>,
/// A line describing reported work.
    #[serde(default, rename = "WorkReportLine")]
    pub work_report_line: Vec<cac::WorkReportLine>,
}
