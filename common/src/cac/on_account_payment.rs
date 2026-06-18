#[derive(Debug, Deserialize, Serialize)]
pub struct OnAccountPayment {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "EstimatedConsumedQuantity")]
    pub estimated_consumed_quantity: super::cct::QuantityType,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<PaymentTerms>,
}
