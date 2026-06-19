#[derive(Debug, Deserialize, Serialize)]
pub struct RemittanceAdviceLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "DebitLineAmount")]
    pub debit_line_amount: Option<cct::Amount>,
    #[serde(default, rename = "CreditLineAmount")]
    pub credit_line_amount: Option<cct::Amount>,
    #[serde(default, rename = "BalanceAmount")]
    pub balance_amount: Option<cct::Amount>,
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: Option<cct::Code>,
    #[serde(default, rename = "InvoicingPartyReference")]
    pub invoicing_party_reference: Option<cct::Text>,
    #[serde(default, rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: Option<SupplierParty>,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: Option<CustomerParty>,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<CustomerParty>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<SupplierParty>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<CustomerParty>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<Party>,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: Option<Party>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<Period>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<BillingReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Option<ExchangeRate>,
}
