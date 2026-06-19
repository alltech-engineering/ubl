#[derive(Debug, Deserialize, Serialize)]
/// A scheduled prepayment (on-account payment) for a estimated utility consumption
///
/// UBL Dictionary Entry Name: `On Account Payment. Details`
///
/// Generated from XSD type `OnAccountPaymentType`.
pub struct OnAccountPayment {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The estimated consumed quantity covered by the payment.
    #[serde(rename = "EstimatedConsumedQuantity")]
    pub estimated_consumed_quantity: cct::Quantity,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A specification of payment terms associated with this payment.
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<PaymentTerms>,
}
