use serde::{Deserialize, Serialize};


include!("terms.rs");
include!("mandate.rs");
include!("means.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Payment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "PaidAmount")]
    pub paid_amount: Option<cct::Amount>,
    #[serde(default, rename = "PaidCashAmount")]
    pub paid_cash_amount: Option<cct::Amount>,
    #[serde(default, rename = "CashChangeAmount")]
    pub cash_change_amount: Option<cct::Amount>,
    #[serde(default, rename = "ReceivedDate")]
    pub received_date: Option<udt::DateTime>,
    #[serde(default, rename = "PaidDate")]
    pub paid_date: Option<udt::DateTime>,
    #[serde(default, rename = "PaidTime")]
    pub paid_time: Option<udt::DateTime>,
    #[serde(default, rename = "InstructionID")]
    pub instruction_id: Option<cct::Identifier>,
    #[serde(default, rename = "MerchantID")]
    pub merchant_id: Option<cct::Identifier>,
    #[serde(default, rename = "AuthorizationID")]
    pub authorization_id: Option<cct::Identifier>,
    #[serde(default, rename = "TransactionID")]
    pub transaction_id: Option<cct::Identifier>,
    #[serde(default, rename = "PaymentTerminalID")]
    pub payment_terminal_id: Option<cct::Identifier>,
    #[serde(default, rename = "StatusCode")]
    pub status_code: Option<cct::Code>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Option<crate::ExchangeRate>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<crate::BillingReference>,
    #[serde(default, rename = "RemittanceDocumentReference")]
    pub remittance_document_reference: Option<crate::DocumentReference>,
}
