#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentMeans {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(rename = "PaymentMeansCode")]
    pub payment_means_code: cct::Code,
    #[serde(default, rename = "PaymentMeansDescription")]
    pub payment_means_description: Vec<cct::Text>,
    #[serde(default, rename = "PaymentDueDate")]
    pub payment_due_date: Option<udt::DateTime>,
    #[serde(default, rename = "PaymentChannelCode")]
    pub payment_channel_code: Option<cct::Code>,
    #[serde(default, rename = "PaymentRailID")]
    pub payment_rail_id: Option<cct::Identifier>,
    #[serde(default, rename = "PaymentPlatformID")]
    pub payment_platform_id: Option<cct::Identifier>,
    #[serde(default, rename = "InstructionID")]
    pub instruction_id: Option<cct::Identifier>,
    #[serde(default, rename = "InstructionNote")]
    pub instruction_note: Vec<cct::Text>,
    #[serde(default, rename = "PaymentID")]
    pub payment_id: Vec<cct::Identifier>,
    #[serde(default, rename = "ChargeBearerCode")]
    pub charge_bearer_code: Option<cct::Code>,
    #[serde(default, rename = "ServiceLevelCode")]
    pub service_level_code: Option<cct::Code>,
    #[serde(default, rename = "CardAccount")]
    pub card_account: Vec<crate::CardAccount>,
    #[serde(default, rename = "PayerFinancialAccount")]
    pub payer_financial_account: Option<crate::FinancialAccount>,
    #[serde(default, rename = "PayeeFinancialAccount")]
    pub payee_financial_account: Option<crate::FinancialAccount>,
    #[serde(default, rename = "CreditAccount")]
    pub credit_account: Option<crate::CreditAccount>,
    #[serde(default, rename = "PaymentMandate")]
    pub payment_mandate: Option<PaymentMandate>,
    #[serde(default, rename = "TradeFinancing")]
    pub trade_financing: Option<crate::TradeFinancing>,
    #[serde(default, rename = "RemittanceDocumentDistribution")]
    pub remittance_document_distribution: Vec<crate::DocumentDistribution>,
    #[serde(default, rename = "PaymentInstructionAttachment")]
    pub payment_instruction_attachment: Option<crate::Attachment>,
}
