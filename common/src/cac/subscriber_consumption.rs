#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriberConsumption {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ConsumptionID")]
    pub consumption_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SpecificationTypeCode")]
    pub specification_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "TotalMeteredQuantity")]
    pub total_metered_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "SubscriberParty")]
    pub subscriber_party: Option<Party>,
    #[serde(rename = "UtilityConsumptionPoint")]
    pub utility_consumption_point: ConsumptionPoint,
    #[serde(default, rename = "OnAccountPayment")]
    pub on_account_payment: Vec<OnAccountPayment>,
    #[serde(default, rename = "Consumption")]
    pub consumption: Option<Consumption>,
    #[serde(default, rename = "SupplierConsumption")]
    pub supplier_consumption: Vec<SupplierConsumption>,
}
