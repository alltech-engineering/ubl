use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 StockAvailabilityReport document type.
/// A report on the availability of stock.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StockAvailabilityReport {
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
    pub copy_indicator: Option<ubl_common::cbc::CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<ubl_common::cbc::UUID>,
    pub issue_date: ubl_common::cbc::IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<ubl_common::cbc::IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ubl_common::cbc::Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_currency_code: Option<ubl_common::cbc::DocumentCurrencyCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inventory_period: Option<InventoryPeriod>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_reference: Vec<ubl_common::cac::DocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    pub seller_supplier_party: SellerSupplierParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retailer_customer_party: Option<RetailerCustomerParty>,
    pub inventory_reporting_party: InventoryReportingParty,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stock_availability_report_line: Vec<StockAvailabilityReportLine>,
}

// ── Inline CAC types ──

/// UBL InventoryPeriod — a Period with this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventoryPeriod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<ubl_common::cac::Period>,
}

/// UBL SellerSupplierParty — a SupplierParty playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SellerSupplierParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::SupplierParty>,
}

/// UBL RetailerCustomerParty — a CustomerParty playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetailerCustomerParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::CustomerParty>,
}

/// UBL InventoryReportingParty — a Party playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InventoryReportingParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Party>,
}

/// UBL 2.5 StockAvailabilityReportLine — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StockAvailabilityReportLine {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}
