#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a payment mandate.
///
/// UBL Dictionary Entry Name: `Payment Mandate. Details`
///
/// Generated from XSD type `PaymentMandateType`.
pub struct PaymentMandate {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this payment mandate.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code signifying the type of this payment mandate.
    #[serde(default, rename = "MandateTypeCode")]
    pub mandate_type_code: Option<cct::Code>,
/// The number of maximum payment instructions allowed within the validity period.
    #[serde(default, rename = "MaximumPaymentInstructionsNumeric")]
    pub maximum_payment_instructions_numeric: Option<cct::Numeric>,
/// The maximum amount to be paid within a single instruction.
    #[serde(default, rename = "MaximumPaidAmount")]
    pub maximum_paid_amount: Option<cct::Amount>,
/// An identifier for a signature applied by a signatory party.
    #[serde(default, rename = "SignatureID")]
    pub signature_id: Option<cct::Identifier>,
/// The Party, if different from the debtor, that makes the Payment.
    #[serde(default, rename = "PayerParty")]
    pub payer_party: Option<crate::Party>,
/// The payer's financial account.
    #[serde(default, rename = "PayerFinancialAccount")]
    pub payer_financial_account: Option<crate::FinancialAccount>,
/// The period during which this mandate is valid.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<crate::Period>,
/// The period of the reverse payment.
    #[serde(default, rename = "PaymentReversalPeriod")]
    pub payment_reversal_period: Option<crate::Period>,
/// A clause applicable to this payment mandate.
    #[serde(default, rename = "Clause")]
    pub clause: Vec<crate::Clause>,
}
