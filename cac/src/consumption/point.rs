#[derive(Debug, Deserialize, Serialize)]
/// A class to define the point of consumption for a utility, such as a meter.
///
/// UBL Dictionary Entry Name: `Consumption Point. Details`
///
/// Generated from XSD type `ConsumptionPointType`.
pub struct ConsumptionPoint {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this point of consumption.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// Text describing this consumption point.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// An identifier for the subscriber responsible for the consumption at this consumption point.
    #[serde(default, rename = "SubscriberID")]
    pub subscriber_id: Option<cct::Identifier>,
/// The type of subscriber, expressed as text.
    #[serde(default, rename = "SubscriberType")]
    pub subscriber_type: Option<cct::Text>,
/// The type of subscriber, expressed as a code.
    #[serde(default, rename = "SubscriberTypeCode")]
    pub subscriber_type_code: Option<cct::Code>,
/// The total quantity delivered, calculated at this consumption point.
    #[serde(default, rename = "TotalDeliveredQuantity")]
    pub total_delivered_quantity: Option<cct::Quantity>,
/// The address of this consumption point.
    #[serde(default, rename = "Address")]
    pub address: Option<crate::Address>,
/// Access information for the website of this consumption point.
    #[serde(default, rename = "WebSiteAccess")]
    pub web_site_access: Option<crate::WebSiteAccess>,
/// A meter at this consumption point.
    #[serde(default, rename = "UtilityMeter")]
    pub utility_meter: Vec<crate::Meter>,
}
