#[derive(Debug, Deserialize, Serialize)]
pub struct NotificationRequirement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "NotificationTypeCode")]
    pub notification_type_code: cct::Code,
    #[serde(default, rename = "PostEventNotificationDurationMeasure")]
    pub post_event_notification_duration_measure: Option<cct::Measure>,
    #[serde(default, rename = "PreEventNotificationDurationMeasure")]
    pub pre_event_notification_duration_measure: Option<cct::Measure>,
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: Vec<Party>,
    #[serde(default, rename = "NotificationPeriod")]
    pub notification_period: Vec<Period>,
    #[serde(default, rename = "NotificationLocation")]
    pub notification_location: Vec<Location>,
}
