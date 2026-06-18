#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PaymentMeansID")]
    pub payment_means_id: Vec<super::cct::IdentifierType>,
    #[serde(default, rename = "PrepaidPaymentReferenceID")]
    pub prepaid_payment_reference_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "ReferenceEventCode")]
    pub reference_event_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "SettlementDiscountPercent")]
    pub settlement_discount_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "PenaltySurchargePercent")]
    pub penalty_surcharge_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "PaymentPercent")]
    pub payment_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "Amount")]
    pub amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "SettlementDiscountAmount")]
    pub settlement_discount_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "PenaltyAmount")]
    pub penalty_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "PaymentTermsDetailsURI")]
    pub payment_terms_details_uri: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PaymentDueDate")]
    pub payment_due_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "InstallmentDueDate")]
    pub installment_due_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "InvoicingPartyReference")]
    pub invoicing_party_reference: Option<super::cct::TextType>,
    #[serde(default, rename = "SettlementPeriod")]
    pub settlement_period: Option<Period>,
    #[serde(default, rename = "PenaltyPeriod")]
    pub penalty_period: Option<Period>,
    #[serde(default, rename = "PenaltyInterestRate")]
    pub penalty_interest_rate: Option<InterestRate>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Option<ExchangeRate>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
}
