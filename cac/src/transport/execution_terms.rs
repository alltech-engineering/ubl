#[derive(Debug, Deserialize, Serialize)]
/// A class to describe terms applying to a transport execution plan.
///
/// UBL Dictionary Entry Name: `Transport Execution Terms. Details`
///
/// Generated from XSD type `TransportExecutionTermsType`.
pub struct TransportExecutionTerms {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// Text describing special terms specified by the transport user.
    #[serde(default, rename = "TransportUserSpecialTerms")]
    pub transport_user_special_terms: Vec<cct::Text>,
/// Text describing special terms specified by the transport service provider.
    #[serde(default, rename = "TransportServiceProviderSpecialTerms")]
    pub transport_service_provider_special_terms: Vec<cct::Text>,
/// Text describing conditions applying to a change of these transport execution terms.
    #[serde(default, rename = "ChangeConditions")]
    pub change_conditions: Vec<cct::Text>,
/// Payment terms associated with the transportation service.
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: Vec<crate::PaymentTerms>,
/// Delivery terms (e.g., Incoterms) associated with the transportation service.
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: Vec<crate::DeliveryTerms>,
/// Terms relating to payment of applicable bonuses associated with the transport service.
    #[serde(default, rename = "BonusPaymentTerms")]
    pub bonus_payment_terms: Option<crate::PaymentTerms>,
/// Terms of payment applying to a commission specified in the transport execution plan.
    #[serde(default, rename = "CommissionPaymentTerms")]
    pub commission_payment_terms: Option<crate::PaymentTerms>,
/// Terms of payment applying to a penalty specified in the transport execution plan.
    #[serde(default, rename = "PenaltyPaymentTerms")]
    pub penalty_payment_terms: Option<crate::PaymentTerms>,
/// An environmental emission resulting from the transportation service.
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: Vec<crate::EnvironmentalEmission>,
/// A notification requirement related to the transportation service; e.g., a requirement that the
/// transport user will be notified when goods are ready for pickup.
    #[serde(default, rename = "NotificationRequirement")]
    pub notification_requirement: Vec<crate::NotificationRequirement>,
/// Payment terms for the service charge associated with the transport service.
    #[serde(default, rename = "ServiceChargePaymentTerms")]
    pub service_charge_payment_terms: Option<crate::PaymentTerms>,
}
