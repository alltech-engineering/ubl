#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a means of payment.
///
/// UBL Dictionary Entry Name: `Payment Means. Details`
///
/// Generated from XSD type `PaymentMeansType`.
pub struct PaymentMeans {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this means of payment.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code signifying the type of this means of payment.
    #[serde(rename = "PaymentMeansCode")]
    pub payment_means_code: cct::Code,
/// A description of this means of payment.
    #[serde(default, rename = "PaymentMeansDescription")]
    pub payment_means_description: Vec<cct::Text>,
/// The date on which payment is due for this means of payment.
    #[serde(default, rename = "PaymentDueDate")]
    pub payment_due_date: Option<udt::DateTime>,
/// A code signifying the Payment Channel for this Payment Means.
    #[serde(default, rename = "PaymentChannelCode")]
    pub payment_channel_code: Option<cct::Code>,
/// An identifier of the Payment Rail or network through which the Payment is executed.
    #[serde(default, rename = "PaymentRailID")]
    pub payment_rail_id: Option<cct::Identifier>,
/// An identifier of the Payment Platform on which the Payment is executed or received.
    #[serde(default, rename = "PaymentPlatformID")]
    pub payment_platform_id: Option<cct::Identifier>,
/// An identifier for the payment instruction.
    #[serde(default, rename = "InstructionID")]
    pub instruction_id: Option<cct::Identifier>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "InstructionNote")]
    pub instruction_note: Vec<cct::Text>,
/// An identifier for a payment made using this means of payment.
    #[serde(default, rename = "PaymentID")]
    pub payment_id: Vec<cct::Identifier>,
/// A code signifying which party or parties will assume the charges and fees associated with the
/// payment using this payment means.
    #[serde(default, rename = "ChargeBearerCode")]
    pub charge_bearer_code: Option<cct::Code>,
/// A code signifying an agreed service level for the type of payment associated with this payment
/// means.
    #[serde(default, rename = "ServiceLevelCode")]
    pub service_level_code: Option<cct::Code>,
/// A credit card, debit card, or charge card account that constitutes this means of payment.
    #[serde(default, rename = "CardAccount")]
    pub card_account: Vec<crate::CardAccount>,
/// The payer's financial account.
    #[serde(default, rename = "PayerFinancialAccount")]
    pub payer_financial_account: Option<crate::FinancialAccount>,
/// The payee's financial account.
    #[serde(default, rename = "PayeeFinancialAccount")]
    pub payee_financial_account: Option<crate::FinancialAccount>,
/// A credit account associated with this means of payment.
    #[serde(default, rename = "CreditAccount")]
    pub credit_account: Option<crate::CreditAccount>,
/// The payment mandate associated with this means of payment.
    #[serde(default, rename = "PaymentMandate")]
    pub payment_mandate: Option<PaymentMandate>,
/// A trade finance agreement applicable to this means of payment.
    #[serde(default, rename = "TradeFinancing")]
    pub trade_financing: Option<crate::TradeFinancing>,
/// A person or entity who will receive the remittance advice information about the payment associated
/// with this payment means.
    #[serde(default, rename = "RemittanceDocumentDistribution")]
    pub remittance_document_distribution: Vec<crate::DocumentDistribution>,
/// Structured payment instruction information including such intended for rendering as a scannable
/// symbol (e.g., QR-code) or for automated processing by external systems.
    #[serde(default, rename = "PaymentInstructionAttachment")]
    pub payment_instruction_attachment: Option<crate::Attachment>,
}
