#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentMandate {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "MandateTypeCode")]
    pub mandate_type_code: Option<cct::Code>,
    #[serde(default, rename = "MaximumPaymentInstructionsNumeric")]
    pub maximum_payment_instructions_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "MaximumPaidAmount")]
    pub maximum_paid_amount: Option<cct::Amount>,
    #[serde(default, rename = "SignatureID")]
    pub signature_id: Option<cct::Identifier>,
    #[serde(default, rename = "PayerParty")]
    pub payer_party: Option<crate::Party>,
    #[serde(default, rename = "PayerFinancialAccount")]
    pub payer_financial_account: Option<crate::FinancialAccount>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<crate::Period>,
    #[serde(default, rename = "PaymentReversalPeriod")]
    pub payment_reversal_period: Option<crate::Period>,
    #[serde(default, rename = "Clause")]
    pub clause: Vec<crate::Clause>,
}
