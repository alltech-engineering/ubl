#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentMandate {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "MandateTypeCode")]
    pub mandate_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "MaximumPaymentInstructionsNumeric")]
    pub maximum_payment_instructions_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "MaximumPaidAmount")]
    pub maximum_paid_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "SignatureID")]
    pub signature_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PayerParty")]
    pub payer_party: Option<Party>,
    #[serde(default, rename = "PayerFinancialAccount")]
    pub payer_financial_account: Option<FinancialAccount>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
    #[serde(default, rename = "PaymentReversalPeriod")]
    pub payment_reversal_period: Option<Period>,
    #[serde(default, rename = "Clause")]
    pub clause: Vec<Clause>,
}
