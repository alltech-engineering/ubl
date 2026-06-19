#[derive(Debug, Deserialize, Serialize)]
pub struct TransportExecutionTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "TransportUserSpecialTerms")]
    pub transport_user_special_terms: Vec<cct::Text>,
    #[serde(default, rename = "TransportServiceProviderSpecialTerms")]
    pub transport_service_provider_special_terms: Vec<cct::Text>,
    #[serde(default, rename = "ChangeConditions")]
    pub change_conditions: Vec<cct::Text>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<crate::PaymentTerms>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Vec<crate::DeliveryTerms>,
    #[serde(default, rename = "BonusPaymentTerms")]
    pub bonus_payment_terms: Option<crate::PaymentTerms>,
    #[serde(default, rename = "CommissionPaymentTerms")]
    pub commission_payment_terms: Option<crate::PaymentTerms>,
    #[serde(default, rename = "PenaltyPaymentTerms")]
    pub penalty_payment_terms: Option<crate::PaymentTerms>,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<crate::EnvironmentalEmission>,
    #[serde(default, rename = "NotificationRequirement")]
    pub notification_requirement: Vec<crate::NotificationRequirement>,
    #[serde(default, rename = "ServiceChargePaymentTerms")]
    pub service_charge_payment_terms: Option<crate::PaymentTerms>,
}
