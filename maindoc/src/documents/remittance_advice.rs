#[derive(Debug, Deserialize, Serialize)]
pub struct RemittanceAdvice {
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
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: Option<cct::Code>,
    #[serde(default, rename = "TotalDebitAmount")]
    pub total_debit_amount: Option<cct::Amount>,
    #[serde(default, rename = "TotalCreditAmount")]
    pub total_credit_amount: Option<cct::Amount>,
    #[serde(default, rename = "TotalPaymentAmount")]
    pub total_payment_amount: Option<cct::Amount>,
    #[serde(default, rename = "PaymentOrderReference")]
    pub payment_order_reference: Option<cct::Text>,
    #[serde(default, rename = "PayerReference")]
    pub payer_reference: Option<cct::Text>,
    #[serde(default, rename = "InvoicingPartyReference")]
    pub invoicing_party_reference: Option<cct::Text>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<cac::Period>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Option<cac::BillingReference>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(rename = "AccountingCustomerParty")]
    pub accounting_customer_party: cac::CustomerParty,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierParty,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: Option<cac::Party>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Option<cac::PaymentMeans>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<cac::TaxTotal>,
    #[serde(default, rename = "RemittanceAdviceLine")]
    pub remittance_advice_line: Vec<cac::RemittanceAdviceLine>,
}
