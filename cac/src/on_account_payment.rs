#[derive(Debug, Deserialize, Serialize)]
pub struct OnAccountPayment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "EstimatedConsumedQuantity")]
    pub estimated_consumed_quantity: cct::Quantity,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<PaymentTerms>,
}
