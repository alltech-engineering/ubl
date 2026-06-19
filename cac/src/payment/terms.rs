#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "PaymentMeansID")]
    pub payment_means_id: Vec<cct::Identifier>,
    #[serde(default, rename = "PrepaidPaymentReferenceID")]
    pub prepaid_payment_reference_id: Option<cct::Identifier>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "ReferenceEventCode")]
    pub reference_event_code: Option<cct::Code>,
    #[serde(default, rename = "SettlementDiscountPercent")]
    pub settlement_discount_percent: Option<cct::Numeric>,
    #[serde(default, rename = "PenaltySurchargePercent")]
    pub penalty_surcharge_percent: Option<cct::Numeric>,
    #[serde(default, rename = "PaymentPercent")]
    pub payment_percent: Option<cct::Numeric>,
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
    #[serde(default, rename = "SettlementDiscountAmount")]
    pub settlement_discount_amount: Option<cct::Amount>,
    #[serde(default, rename = "PenaltyAmount")]
    pub penalty_amount: Option<cct::Amount>,
    #[serde(default, rename = "PaymentTermsDetailsURI")]
    pub payment_terms_details_uri: Option<cct::Identifier>,
    #[serde(default, rename = "PaymentDueDate")]
    pub payment_due_date: Option<udt::DateTime>,
    #[serde(default, rename = "InstallmentDueDate")]
    pub installment_due_date: Option<udt::DateTime>,
    #[serde(default, rename = "InvoicingPartyReference")]
    pub invoicing_party_reference: Option<cct::Text>,
    #[serde(default, rename = "SettlementPeriod")]
    pub settlement_period: Option<crate::Period>,
    #[serde(default, rename = "PenaltyPeriod")]
    pub penalty_period: Option<crate::Period>,
    #[serde(default, rename = "PenaltyInterestRate")]
    pub penalty_interest_rate: Option<crate::InterestRate>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Option<crate::ExchangeRate>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<crate::Period>,
}
