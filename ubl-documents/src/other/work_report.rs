use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 WorkReport document type.
/// Work completion report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkReport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<ubl_common::cbc::UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<ubl_common::cbc::CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ubl_common::cbc::ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ubl_common::cbc::ProfileExecutionID>,
    pub id: ubl_common::cbc::ID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<ubl_common::cbc::UUID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<ubl_common::cbc::VersionID>,
    pub issue_date: ubl_common::cbc::IssueDate,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ubl_common::cbc::Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<ubl_common::cbc::AccountingCostCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<ubl_common::cbc::AccountingCost>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub work_quantity_total: Vec<WorkQuantityTotal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reported_period: Option<ReportedPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_reference: Option<ubl_common::cac::OrderReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_reference: Option<ubl_common::cac::ProjectReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub billing_reference: Vec<ubl_common::cac::billing_reference::BillingReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    pub seller_supplier_party: SellerSupplierParty,
    pub buyer_customer_party: BuyerCustomerParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_party: Option<ApproverParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowance_charge: Vec<ubl_common::cac::AllowanceCharge>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tax_total: Vec<ubl_common::cac::TaxTotal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_monetary_total: Option<StatementMonetaryTotal>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub work_report_line: Vec<WorkReportLine>,
}

// ── Inline CAC types ──

/// UBL 2.5 WorkQuantityTotal — TODO: define fields from CAC schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkQuantityTotal {
    // TODO: Define fields from UBL 2.5 CAC schema
}

/// UBL ReportedPeriod — a Period with this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReportedPeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<ubl_common::cac::Period>,
}

/// UBL 2.5 AdditionalDocumentReference — TODO: define fields from CAC schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalDocumentReference {
    // TODO: Define fields from UBL 2.5 CAC schema
}

/// UBL SellerSupplierParty — a SupplierParty playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SellerSupplierParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::SupplierParty>,
    // TODO: Add role-specific fields from UBL 2.5 CAC schema
}

/// UBL BuyerCustomerParty — a CustomerParty playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuyerCustomerParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::CustomerParty>,
    // TODO: Add role-specific fields from UBL 2.5 CAC schema
}

/// UBL ApproverParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApproverParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
    // TODO: Add role-specific fields from UBL 2.5 CAC schema
}

/// UBL 2.5 StatementMonetaryTotal — TODO: define fields from CAC schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatementMonetaryTotal {
    // TODO: Define fields from UBL 2.5 CAC schema
}

/// UBL 2.5 WorkReportLine — TODO: define fields from CAC schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkReportLine {
    // TODO: Define fields from UBL 2.5 CAC schema
}
