use serde::{Deserialize, Serialize};

// Re-exports for convenience

/// UBL 2.5 PurchaseReceipt document type.
/// Receipt for a purchase transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseReceipt {
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
    pub issue_date: ubl_common::cbc::IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<ubl_common::cbc::IssueTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_date: Option<ubl_common::cbc::TransactionDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_time: Option<ubl_common::cbc::TransactionTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<ubl_common::cbc::PurchaseDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchase_time: Option<ubl_common::cbc::PurchaseTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<ubl_common::cbc::Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_currency_code: Option<ubl_common::cbc::DocumentCurrencyCode>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purchase_reference: Vec<PurchaseReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sales_document_reference: Option<SalesDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<ubl_common::cac::Signature>,
    pub accounting_supplier_party: AccountingSupplierParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_customer_party: Option<AccountingCustomerParty>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cashier_contact: Option<CashierContact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash_register: Option<CashRegister>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point_of_sale_location: Option<PointOfSaleLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point_of_sale_contact: Option<PointOfSaleContact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<ubl_common::cac::Delivery>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment: Vec<ubl_common::cac::Payment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payment_means: Vec<ubl_common::cac::PaymentMeans>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowance_charge: Vec<ubl_common::cac::AllowanceCharge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_exchange_rate: Option<TaxExchangeRate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_exchange_rate: Option<PricingExchangeRate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tax_total: Vec<ubl_common::cac::TaxTotal>,
    pub legal_monetary_total: ubl_common::cac::MonetaryTotal,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purchase_receipt_line: Vec<PurchaseReceiptLine>,
}

// ── Inline CAC types ──

/// UBL 2.5 PurchaseReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}

/// UBL 2.5 SalesDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SalesDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}

/// UBL 2.5 AdditionalDocumentReference — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionalDocumentReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<ubl_common::cac::document_reference::DocumentReference>,
}

/// UBL AccountingSupplierParty — a SupplierParty playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountingSupplierParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::SupplierParty>,
}

/// UBL AccountingCustomerParty — a CustomerParty playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountingCustomerParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::CustomerParty>,
}

/// UBL CashierContact — a Contact playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CashierContact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Contact>,
}

/// UBL 2.5 CashRegister — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CashRegister {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}

/// UBL 2.5 PointOfSaleLocation — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointOfSaleLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<ubl_common::cac::location::Location>,
}

/// UBL PointOfSaleContact — a Contact playing this specific role.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointOfSaleContact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ubl_common::cac::Contact>,
}

/// UBL 2.5 TaxExchangeRate — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaxExchangeRate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<ubl_common::cac::exchange_rate::ExchangeRate>,
}

/// UBL 2.5 PricingExchangeRate — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PricingExchangeRate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<ubl_common::cac::exchange_rate::ExchangeRate>,
}

/// UBL 2.5 PurchaseReceiptLine — real field definitions from UBL 2.5 CAC schema
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PurchaseReceiptLine {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ubl_common::cbc::ID>,
}
