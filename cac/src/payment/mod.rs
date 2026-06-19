use serde::{Deserialize, Serialize};


include!("terms.rs");
include!("mandate.rs");
include!("means.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a payment.
///
/// UBL Dictionary Entry Name: `Payment. Details`
///
/// Generated from XSD type `PaymentType`.
pub struct Payment {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this payment.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The amount of this payment.
    #[serde(default, rename = "PaidAmount")]
    pub paid_amount: Option<cct::Amount>,
/// The amount given by the customer in cash or cash equivalents, if different from the payable amount.
/// The Paid Amount = Paid Cash Amount - Cash Change Amount.
    #[serde(default, rename = "PaidCashAmount")]
    pub paid_cash_amount: Option<cct::Amount>,
/// The change returned to the customer when the paid cash amount is more than the payable amount.
    #[serde(default, rename = "CashChangeAmount")]
    pub cash_change_amount: Option<cct::Amount>,
/// The date on which this payment was received.
    #[serde(default, rename = "ReceivedDate")]
    pub received_date: Option<udt::DateTime>,
/// The date on which this payment was made.
    #[serde(default, rename = "PaidDate")]
    pub paid_date: Option<udt::DateTime>,
/// The time at which this payment was made.
    #[serde(default, rename = "PaidTime")]
    pub paid_time: Option<udt::DateTime>,
/// An identifier for the payment instruction.
    #[serde(default, rename = "InstructionID")]
    pub instruction_id: Option<cct::Identifier>,
/// An identifier for the merchant who handled the payment.
    #[serde(default, rename = "MerchantID")]
    pub merchant_id: Option<cct::Identifier>,
/// The authorization identifier for this payment.
    #[serde(default, rename = "AuthorizationID")]
    pub authorization_id: Option<cct::Identifier>,
/// The transaction identifier for this payment.
    #[serde(default, rename = "TransactionID")]
    pub transaction_id: Option<cct::Identifier>,
/// An identifier for the payment terminal used for this payment.
    #[serde(default, rename = "PaymentTerminalID")]
    pub payment_terminal_id: Option<cct::Identifier>,
/// A code signifying the status of the Payment (e.g., planned, in process, executed).
    #[serde(default, rename = "StatusCode")]
    pub status_code: Option<cct::Code>,
/// The exchange rate applicable to this payment, if the payment currency differs from the document
/// currency.
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Option<crate::ExchangeRate>,
/// A reference to a billing document to which this Payment relates.
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<crate::BillingReference>,
/// A reference to a Remittance Advice document associated with this Payment.
    #[serde(default, rename = "RemittanceDocumentReference")]
    pub remittance_document_reference: Option<crate::DocumentReference>,
}
