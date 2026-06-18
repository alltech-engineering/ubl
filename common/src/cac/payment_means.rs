#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentMeans {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(rename = "PaymentMeansCode")]
    pub payment_means_code: super::cct::CodeType,
    #[serde(default, rename = "PaymentMeansDescription")]
    pub payment_means_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "PaymentDueDate")]
    pub payment_due_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "PaymentChannelCode")]
    pub payment_channel_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "PaymentRailID")]
    pub payment_rail_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PaymentPlatformID")]
    pub payment_platform_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "InstructionID")]
    pub instruction_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "InstructionNote")]
    pub instruction_note: Vec<super::cct::TextType>,
    #[serde(default, rename = "PaymentID")]
    pub payment_id: Vec<super::cct::IdentifierType>,
    #[serde(default, rename = "ChargeBearerCode")]
    pub charge_bearer_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ServiceLevelCode")]
    pub service_level_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CardAccount")]
    pub card_account: Vec<CardAccount>,
    #[serde(default, rename = "PayerFinancialAccount")]
    pub payer_financial_account: Option<FinancialAccount>,
    #[serde(default, rename = "PayeeFinancialAccount")]
    pub payee_financial_account: Option<FinancialAccount>,
    #[serde(default, rename = "CreditAccount")]
    pub credit_account: Option<CreditAccount>,
    #[serde(default, rename = "PaymentMandate")]
    pub payment_mandate: Option<PaymentMandate>,
    #[serde(default, rename = "TradeFinancing")]
    pub trade_financing: Option<TradeFinancing>,
    #[serde(default, rename = "RemittanceDocumentDistribution")]
    pub remittance_document_distribution: Vec<DocumentDistribution>,
    #[serde(default, rename = "PaymentInstructionAttachment")]
    pub payment_instruction_attachment: Option<Attachment>,
}
