#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriberConsumption {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ConsumptionID")]
    pub consumption_id: Option<cct::Identifier>,
    #[serde(default, rename = "SpecificationTypeCode")]
    pub specification_type_code: Option<cct::Code>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "TotalMeteredQuantity")]
    pub total_metered_quantity: Option<cct::Quantity>,
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
