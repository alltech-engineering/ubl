#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionPoint {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "SubscriberID")]
    pub subscriber_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SubscriberType")]
    pub subscriber_type: Option<super::cct::TextType>,
    #[serde(default, rename = "SubscriberTypeCode")]
    pub subscriber_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TotalDeliveredQuantity")]
    pub total_delivered_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "Address")]
    pub address: Option<Address>,
    #[serde(default, rename = "WebSiteAccess")]
    pub web_site_access: Option<WebSiteAccess>,
    #[serde(default, rename = "UtilityMeter")]
    pub utility_meter: Vec<Meter>,
}
