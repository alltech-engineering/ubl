#[derive(Debug, Deserialize, Serialize)]
pub struct Payment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PaidAmount")]
    pub paid_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "PaidCashAmount")]
    pub paid_cash_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "CashChangeAmount")]
    pub cash_change_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "ReceivedDate")]
    pub received_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "PaidDate")]
    pub paid_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "PaidTime")]
    pub paid_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "InstructionID")]
    pub instruction_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "MerchantID")]
    pub merchant_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AuthorizationID")]
    pub authorization_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TransactionID")]
    pub transaction_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PaymentTerminalID")]
    pub payment_terminal_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "StatusCode")]
    pub status_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Option<ExchangeRate>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<BillingReference>,
    #[serde(default, rename = "RemittanceDocumentReference")]
    pub remittance_document_reference: Option<DocumentReference>,
}
