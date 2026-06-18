#[derive(Debug, Deserialize, Serialize)]
pub struct NotificationRequirement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "NotificationTypeCode")]
    pub notification_type_code: super::cct::CodeType,
    #[serde(default, rename = "PostEventNotificationDurationMeasure")]
    pub post_event_notification_duration_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "PreEventNotificationDurationMeasure")]
    pub pre_event_notification_duration_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: Vec<Party>,
    #[serde(default, rename = "NotificationPeriod")]
    pub notification_period: Vec<Period>,
    #[serde(default, rename = "NotificationLocation")]
    pub notification_location: Vec<Location>,
}
