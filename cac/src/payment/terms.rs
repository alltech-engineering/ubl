#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a set of payment terms.
///
/// UBL Dictionary Entry Name: `Payment Terms. Details`
///
/// Generated from XSD type `PaymentTermsType`.
pub struct PaymentTerms {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this set of payment terms.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// An identifier for a means of payment associated with these payment terms.
    #[serde(default, rename = "PaymentMeansID")]
    pub payment_means_id: Vec<cct::Identifier>,
/// An identifier for a reference to a prepaid payment.
    #[serde(default, rename = "PrepaidPaymentReferenceID")]
    pub prepaid_payment_reference_id: Option<cct::Identifier>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the event during which these terms are offered.
    #[serde(default, rename = "ReferenceEventCode")]
    pub reference_event_code: Option<cct::Code>,
/// The percentage for the settlement discount that is offered for payment under these payment terms.
    #[serde(default, rename = "SettlementDiscountPercent")]
    pub settlement_discount_percent: Option<cct::Numeric>,
/// The penalty for payment after the settlement period, expressed as a percentage of the payment.
    #[serde(default, rename = "PenaltySurchargePercent")]
    pub penalty_surcharge_percent: Option<cct::Numeric>,
/// The part of a payment, expressed as a percent, relevant for these payment terms.
    #[serde(default, rename = "PaymentPercent")]
    pub payment_percent: Option<cct::Numeric>,
/// The monetary amount covered by these payment terms.
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
/// The amount of a settlement discount offered for payment under these payment terms.
    #[serde(default, rename = "SettlementDiscountAmount")]
    pub settlement_discount_amount: Option<cct::Amount>,
/// The monetary amount of the penalty for payment after the settlement period.
    #[serde(default, rename = "PenaltyAmount")]
    pub penalty_amount: Option<cct::Amount>,
/// The Uniform Resource Identifier (URI) of a document providing additional details regarding these
/// payment terms.
    #[serde(default, rename = "PaymentTermsDetailsURI")]
    pub payment_terms_details_uri: Option<cct::Identifier>,
/// The due date for these payment terms.
    #[serde(default, rename = "PaymentDueDate")]
    pub payment_due_date: Option<udt::DateTime>,
/// The due date for an installment payment for these payment terms.
    #[serde(default, rename = "InstallmentDueDate")]
    pub installment_due_date: Option<udt::DateTime>,
/// A reference to the payment terms used by the invoicing party. This may have been requested of the
/// payer by the payee to accompany its remittance.
    #[serde(default, rename = "InvoicingPartyReference")]
    pub invoicing_party_reference: Option<cct::Text>,
/// The period during which settlement may occur.
    #[serde(default, rename = "SettlementPeriod")]
    pub settlement_period: Option<crate::Period>,
/// The period during which penalties may apply.
    #[serde(default, rename = "PenaltyPeriod")]
    pub penalty_period: Option<crate::Period>,
/// An interest rate to be applied in case of late payment.
    #[serde(default, rename = "PenaltyInterestRate")]
    pub penalty_interest_rate: Option<crate::InterestRate>,
/// The currency exchange rate for purposes of these payment terms.
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Option<crate::ExchangeRate>,
/// The period during which these payment terms are valid.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<crate::Period>,
}
