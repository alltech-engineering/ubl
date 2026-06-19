#[derive(Debug, Deserialize, Serialize)]
/// A class to define a line in a Remittance Advice.
///
/// UBL Dictionary Entry Name: `Remittance Advice Line. Details`
///
/// Generated from XSD type `RemittanceAdviceLineType`.
pub struct RemittanceAdviceLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this remittance advice line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A universally unique identifier for this remittance advice line.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The amount debited on this remittance advice line.
    #[serde(default, rename = "DebitLineAmount")]
    pub debit_line_amount: Option<cct::Amount>,
/// The amount credited on this remittance advice line.
    #[serde(default, rename = "CreditLineAmount")]
    pub credit_line_amount: Option<cct::Amount>,
/// The monetary balance associated with this remittance advice line.
    #[serde(default, rename = "BalanceAmount")]
    pub balance_amount: Option<cct::Amount>,
/// A code signifying the business purpose for this payment.
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: Option<cct::Code>,
/// A reference to the order for payment used by the invoicing party. This may have been requested of
/// the payer by the payee to accompany its remittance.
    #[serde(default, rename = "InvoicingPartyReference")]
    pub invoicing_party_reference: Option<cct::Text>,
/// The Accounting Supplier Party related to the remittance information reported on this Remittance
/// Advice Line.
    #[serde(default, rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: Option<SupplierParty>,
/// The Accounting Customer Party related to the remittance information reported on this Remittance
/// Advice Line.
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: Option<CustomerParty>,
/// The buyer associated with this remittance advice line.
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: Option<CustomerParty>,
/// The seller/supplier associated with this remittance advice line.
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: Option<SupplierParty>,
/// The originating party.
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: Option<CustomerParty>,
/// A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: Vec<Party>,
/// The Party who receives the Payment.
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: Option<Party>,
/// An invoice period to which this remittance advice line applies.
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: Vec<Period>,
/// A reference to a billing document associated with this remittance advice line.
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<BillingReference>,
/// A reference to a document associated with this remittance advice line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
/// The rate of exchange between the currency of the Remittance Advice and the currency of the document
/// described in the BillingReference.
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Option<ExchangeRate>,
}
