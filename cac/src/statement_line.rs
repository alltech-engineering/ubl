#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Statement of account.
///
/// UBL Dictionary Entry Name: `Statement Line. Details`
///
/// Generated from XSD type `StatementLineType`.
pub struct StatementLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this statement line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A universally unique identifier for this statement line.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// An indication that this statement line contains an outstanding balance from the previous bill(s)
/// (true) or does not (false).
    #[serde(default, rename = "BalanceBroughtForwardIndicator")]
    pub balance_brought_forward_indicator: Option<udt::Indicator>,
/// The amount debited on this statement line.
    #[serde(default, rename = "DebitLineAmount")]
    pub debit_line_amount: Option<cct::Amount>,
/// The amount credited on this statement line.
    #[serde(default, rename = "CreditLineAmount")]
    pub credit_line_amount: Option<cct::Amount>,
/// The balance amount on this statement line.
    #[serde(default, rename = "BalanceAmount")]
    pub balance_amount: Option<cct::Amount>,
/// A code signifying the business purpose for this payment.
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: Option<cct::Code>,
/// A means of payment associated with this statement line.
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Option<PaymentMeans>,
/// A specification of payment terms associated with this statement line.
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<PaymentTerms>,
/// The buyer associated with this statement line.
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<CustomerParty>,
/// The seller/supplier associated with this statement line.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<SupplierParty>,
/// The originating party.
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<CustomerParty>,
/// A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<Party>,
/// The Accounting Customer Party related to the statement information reported on this Statement Line.
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: Option<CustomerParty>,
/// The Accounting Supplier Party related to the statement information reported on this Statement Line.
    #[serde(default, rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: Option<SupplierParty>,
/// The Party who receives the Payment.
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: Option<Party>,
/// An invoice period to which this statement line applies.
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<Period>,
/// A reference to a billing document associated with this statement line.
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<BillingReference>,
/// A reference to a document associated with this statement line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
/// The rate of exchange between the currency of the Statement and the currency of the document
/// described in the BillingReference.
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Option<ExchangeRate>,
/// A charge or discount price component associated with this statement line.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<AllowanceCharge>,
/// A collected payment.
    #[serde(default, rename = "CollectedPayment")]
    pub collected_payment: Vec<Payment>,
}
