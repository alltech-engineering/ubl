#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionPoint {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "SubscriberID")]
    pub subscriber_id: Option<cct::Identifier>,
    #[serde(default, rename = "SubscriberType")]
    pub subscriber_type: Option<cct::Text>,
    #[serde(default, rename = "SubscriberTypeCode")]
    pub subscriber_type_code: Option<cct::Code>,
    #[serde(default, rename = "TotalDeliveredQuantity")]
    pub total_delivered_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "Address")]
    pub address: Option<crate::Address>,
    #[serde(default, rename = "WebSiteAccess")]
    pub web_site_access: Option<crate::WebSiteAccess>,
    #[serde(default, rename = "UtilityMeter")]
    pub utility_meter: Vec<crate::Meter>,
}
